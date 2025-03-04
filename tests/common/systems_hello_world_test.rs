//! # Hello World System Tests
//! 
//! This module contains tests for the hello_world system functionality.
//! It verifies the basic system setup and execution within the Bevy ECS framework.
//! 
//! # Purpose
//! - Validates that the hello_world system can be properly registered with Bevy
//! - Ensures the system executes without errors
//! - Provides a template for testing other game systems
//! 
//! # Note
//! This is a basic smoke test that ensures the system is correctly 
//! integrated with the Bevy ECS architecture.

use bevy::prelude::*;
use astrocore::systems::hello_world::hello_world;

/// Tests that the hello_world system initializes and runs correctly.
/// 
/// This test:
/// - Creates a minimal Bevy application
/// - Registers the hello_world system in the startup stage
/// - Runs a single update cycle to trigger the system
/// 
/// # Success Criteria
/// The test passes if the hello_world system executes without panicking.
/// The console output should contain "Hello World!" when the test runs.
/// 
/// # Note
/// While this test doesn't assert specific outputs (since hello_world only prints to console),
/// it confirms the system can run within the Bevy lifecycle without crashing.
#[test]
fn system_hello_world_test() {
  // Create a fresh Bevy app for our galactic test mission
  let mut app = App::new();
  
  // Add the hello_world system to the startup stage
  app.add_systems(Startup, hello_world);
  
  // Run the startup systems
  app.update();
}
