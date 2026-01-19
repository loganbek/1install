//! CLI module - command line interface definitions

mod commands;
mod output;

pub use commands::{Cli, Commands, ConfigAction, ShimsAction};
pub use output::{render_search_results, render_backends};
