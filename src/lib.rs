//! # Test Interface
//! 
//! This file serves as an interface for testing purposes.
//! It exposes the same modules as the main application to allow
//! for proper unit testing of components without requiring the
//! entire application to be initialized.
//! 
//! This is not intended to be used as a distributable library,
//! but rather as an implementation detail to facilitate testing.

pub mod systems;
pub mod plugins;
