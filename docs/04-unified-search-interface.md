# Subsystem II: The Unified Federated Search Interface

## 3.1 The Challenge of Output Normalization

Each package manager outputs data in different formats:

| Manager            | Output Format            | Challenge                      |
| ------------------ | ------------------------ | ------------------------------ |
| `apt-cache search` | Raw text                 | Unstructured, requires parsing |
| `winget search`    | Whitespace-aligned table | Column-based parsing needed    |
| `npm`              | JSON API                 | `registry.npmjs.org`           |
| `pip`              | JSON API                 | `pypi.org/pypi/<pkg>/json`     |

## 3.2 The Aggregation Algorithm

1install implements a **Federated Search Aggregator**:

### Step 1: CLI Parsing via jc

Integrates `jc` (JSON Convert) logic to transform CLI output into structured JSON:

```rust
// Example: parsing apt-cache search output
fn parse_apt_search(output: &str) -> Vec<PackageResult> {
    output.lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(2, " - ").collect();
            if parts.len() == 2 {
                Some(PackageResult {
                    name: parts[0].to_string(),
                    description: parts[1].to_string(),
                    source: "apt".to_string(),
                    ..Default::default()
                })
            } else {
                None
            }
        })
        .collect()
}
```

### Step 2: API Querying

For comprehensive results, query external sources:

- **Repology**: Global view of packages across distributions
- **Libraries.io**: Package metadata aggregator

```rust
async fn query_repology(query: &str) -> Vec<PackageResult> {
    let url = format!("https://repology.org/api/v1/project/{}", query);
    // ... fetch and parse
}
```

## 3.3 Search Result Ranking

Results are normalized into a `PackageResult` struct:

```rust
#[derive(Debug, Clone)]
struct PackageResult {
    name: String,
    version: String,
    description: String,
    source: String,  // apt, npm, brew, etc.
    score: f32,      // Ranking score
}
```

### Ranking Criteria

1. **Exact Match**: Exact name matches appear first (highest score)
2. **Backend Priority**: Configurable preference (e.g., `apt > snap`)
3. **Stability**: LTS versions rank above nightly builds
4. **Recency**: Recently updated packages score higher

### Example Search Output

```
$ 1i search python

╭───────────┬─────────┬────────┬─────────────────────────────────╮
│ Package   │ Version │ Source │ Description                     │
├───────────┼─────────┼────────┼─────────────────────────────────┤
│ python3   │ 3.11.4  │ apt    │ Interactive Python interpreter  │
│ python    │ 3.12.0  │ brew   │ Python programming language     │
│ python    │ 3.11.5  │ pyenv  │ Simple Python version manager   │
│ pypy3     │ 7.3.12  │ apt    │ Fast alternative Python impl    │
╰───────────┴─────────┴────────┴─────────────────────────────────╯
```
