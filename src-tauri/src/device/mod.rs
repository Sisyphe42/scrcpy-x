//! Device discovery and management
//! 
//! Handles ADB device discovery, parsing, and device state management.

mod discovery;
mod parser;

pub use discovery::*;
pub use parser::*;
