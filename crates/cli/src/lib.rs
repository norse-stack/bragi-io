// All core functionality is in bragi-io-core
// This CLI acts as a thin wrapper around the core library

// CLI-specific modules
pub mod jre_manager;

// Re-export core types for convenience
pub use bragi_io_core::*;

// Re-export CLI utilities
pub use jre_manager::JreManager;
