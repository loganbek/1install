//! Shim management module

mod generator;
mod registry;

pub use generator::{create_shim, get_shim_dir, ensure_shim_dir, get_path_instruction};
pub use registry::{ShimRegistry, ShimEntry};
