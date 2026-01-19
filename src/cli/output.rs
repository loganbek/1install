//! CLI output formatting

use crate::search::PackageResult;
use comfy_table::{Table, Row, Cell, Color, Attribute};
use comfy_table::presets::UTF8_FULL;

/// Render search results as a formatted table
pub fn render_search_results(results: &[PackageResult], limit: usize) {
    if results.is_empty() {
        println!("No packages found.");
        return;
    }
    
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    
    // Header
    table.set_header(vec![
        Cell::new("Package").add_attribute(Attribute::Bold),
        Cell::new("Version").add_attribute(Attribute::Bold),
        Cell::new("Source").add_attribute(Attribute::Bold),
        Cell::new("Description").add_attribute(Attribute::Bold),
    ]);
    
    // Results (limited)
    for result in results.iter().take(limit) {
        let mut row = Row::new();
        
        // Package name
        row.add_cell(Cell::new(&result.name));
        
        // Version
        row.add_cell(Cell::new(result.version.as_deref().unwrap_or("-")));
        
        // Source with color
        let source_cell = Cell::new(&result.source).fg(source_color(&result.source));
        row.add_cell(source_cell);
        
        // Description (truncated)
        let desc = result.description.as_deref().unwrap_or("-");
        let desc_truncated = if desc.len() > 50 {
            format!("{}...", &desc[..47])
        } else {
            desc.to_string()
        };
        row.add_cell(Cell::new(desc_truncated));
        
        table.add_row(row);
    }
    
    println!("{table}");
    
    let shown = std::cmp::min(results.len(), limit);
    if results.len() > limit {
        println!("\nShowing {} of {} results. Use --limit to see more.", shown, results.len());
    }
}

/// Get color for a backend source
fn source_color(source: &str) -> Color {
    match source {
        "apt" => Color::Green,
        "winget" => Color::Cyan,
        "brew" => Color::Yellow,
        "npm" => Color::Red,
        "pip" | "pipx" => Color::Blue,
        "snap" => Color::Magenta,
        _ => Color::White,
    }
}

/// Render list of available backends
pub fn render_backends(backends: &[&str]) {
    println!("Available backends on this system:\n");
    for backend in backends {
        let _color = source_color(backend);
        // Using comfy_table colors for terminal output is slightly more complex, 
        // for now just prefixing with underscore to clear warning until we implement 
        // specialized colored printing for this list.
        println!("  • {}", backend);
    }
    println!();
}
