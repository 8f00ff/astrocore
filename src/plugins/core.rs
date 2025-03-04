//! # AstroCore Core Plugin
//! Central game systems and initialization.
//! 
//! This plugin manages core game functionality, startup systems,
//! and essential game resources.

use bevy::prelude::*;
use crate::systems::hello_world::hello_world;

pub struct AstroCorePlugin;

/// Core plugin for AstroCore that handles essential game systems.
/// 
/// Manages startup and core game loop functionality, initializing
/// necessary resources and systems.
impl Plugin for AstroCorePlugin {
  
  /// Builds the plugin by adding systems and resources to the Bevy app.
  /// 
  /// # Arguments
  /// * `app` - Mutable reference to the Bevy App being configured
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, hello_world);
  }
  
}
