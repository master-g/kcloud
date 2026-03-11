//! Tools module for Kloud
//!
//! This module contains tool-related types and implementations.

pub mod call;
pub mod traits;

pub use call::{ToolCall, ToolResult};
pub use traits::Tool;
