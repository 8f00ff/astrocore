//! Hello World System Module
//! 
//! Provides a simple startup system for initial game launch demonstration.
//! Primarily used as a basic sanity check and template for more complex systems.
//! 
//! # Purpose
//! - Confirms game engine initialization
//! - Serves as a minimal working example of a Bevy system

/// Prints a classic "Hello World!" message during game startup.
/// 
/// This system is a simple diagnostic that confirms:
/// - Game systems are loading correctly
/// - Basic system registration works
/// - Provides a starting point for more complex system design
/// 
/// # Performance
/// - Minimal computational overhead
/// - Writes to console/log
/// 
/// # Examples
/// ```
/// # use bevy::prelude::*;
/// # use astrocore::systems::hello_world::hello_world;
/// # let mut app = App::new();
/// // Typically added during app startup
/// app.add_systems(Startup, hello_world);
/// ```
pub fn hello_world() {
  println!("Hello World!");
}
