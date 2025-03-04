//! # AstroCore - Space Adventure Game
//! Main entry point for the AstroCore game engine.
//! Handles core game initialization and systems setup.

mod plugins;
mod systems;

use bevy::prelude::*;
use plugins::core::AstroCorePlugin;

/// Launches the game engine and starts the main game loop.
/// Sets up core Bevy systems, plugins, and initial game state.
fn main() {
  App::new()
    .add_plugins(AstroCorePlugin)
    .run();
}
