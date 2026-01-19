//! Search aggregator for federated package discovery

use std::cmp::Ordering;

/// A search result from a package manager
#[derive(Debug, Clone)]
pub struct PackageResult {
    /// Package name
    pub name: String,
    /// Package version (if available)
    pub version: Option<String>,
    /// Short description
    pub description: Option<String>,
    /// Source backend (apt, winget, npm, etc.)
    pub source: String,
    /// Ranking score (higher = better match)
    pub score: f32,
}

impl PackageResult {
    /// Create a new package result
    pub fn new(name: String, source: String) -> Self {
        Self {
            name,
            version: None,
            description: None,
            source,
            score: 0.0,
        }
    }

    /// Set version
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Set description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set score
    pub fn with_score(mut self, score: f32) -> Self {
        self.score = score;
        self
    }
}

impl PartialEq for PackageResult {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.source == other.source
    }
}

impl Eq for PackageResult {}

impl PartialOrd for PackageResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PackageResult {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher score = better, so reverse the comparison
        other.score.partial_cmp(&self.score).unwrap_or(Ordering::Equal)
    }
}

/// Backend priority for ranking results
const BACKEND_PRIORITY: &[&str] = &["apt", "winget", "brew", "snap", "npm", "pip"];

/// Search aggregator collects results from multiple backends
pub struct SearchAggregator;

impl SearchAggregator {
    /// Calculate the score for a package result
    pub fn calculate_score(query: &str, result: &mut PackageResult) {
        let mut score: f32 = 0.0;
        let query_lower = query.to_lowercase();
        let name_lower = result.name.to_lowercase();

        // Exact match bonus (highest priority)
        if name_lower == query_lower {
            score += 100.0;
        }
        // Starts with query bonus
        else if name_lower.starts_with(&query_lower) {
            score += 75.0;
        }
        // Contains query bonus
        else if name_lower.contains(&query_lower) {
            score += 50.0;
        }

        // Backend priority bonus
        if let Some(pos) = BACKEND_PRIORITY.iter().position(|&b| b == result.source) {
            // Earlier in list = higher priority = more bonus
            score += (BACKEND_PRIORITY.len() - pos) as f32 * 5.0;
        }

        // Has version bonus (packages with version info are more reliable)
        if result.version.is_some() {
            score += 5.0;
        }

        // Has description bonus
        if result.description.is_some() {
            score += 2.0;
        }

        result.score = score;
    }

    /// Rank and sort results
    pub fn rank_results(query: &str, results: &mut [PackageResult]) {
        // Calculate scores
        for result in results.iter_mut() {
            Self::calculate_score(query, result);
        }

        // Sort by score (descending)
        results.sort();
    }

    /// Deduplicate results, keeping highest scored version
    pub fn deduplicate(results: &mut Vec<PackageResult>) {
        results.sort();
        results.dedup_by(|a, b| a.name == b.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match_scores_highest() {
        let query = "python";
        let mut exact = PackageResult::new("python".to_string(), "apt".to_string());
        let mut partial = PackageResult::new("python3".to_string(), "apt".to_string());

        SearchAggregator::calculate_score(query, &mut exact);
        SearchAggregator::calculate_score(query, &mut partial);

        assert!(exact.score > partial.score);
    }

    #[test]
    fn test_backend_priority() {
        let query = "git";
        let mut apt_result = PackageResult::new("git".to_string(), "apt".to_string());
        let mut npm_result = PackageResult::new("git".to_string(), "npm".to_string());

        SearchAggregator::calculate_score(query, &mut apt_result);
        SearchAggregator::calculate_score(query, &mut npm_result);

        assert!(apt_result.score > npm_result.score);
    }

    #[test]
    fn test_ranking() {
        let mut results = vec![
            PackageResult::new("python-pip".to_string(), "apt".to_string()),
            PackageResult::new("python".to_string(), "apt".to_string()),
            PackageResult::new("python3".to_string(), "apt".to_string()),
        ];

        SearchAggregator::rank_results("python", &mut results);

        assert_eq!(results[0].name, "python");  // Exact match first
    }
}
