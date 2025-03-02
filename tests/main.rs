//! # Test Suite
//! Main entry point for all tests.
//! 
//! This file serves as the primary entry point when running the 
//! test suite with debugging tools like LLDB.
//! 
//! # Usage
//! Run all tests with LLDB:
//! ```bash
//! cargo test -- --nocapture
//! ```
//! 
//! For debugging with LLDB:
//! ```bash
//! lldb -- cargo test
//! ```

mod common;
