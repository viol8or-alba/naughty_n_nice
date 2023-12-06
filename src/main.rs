#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use setup::InitialSetup;

mod animation;
mod characters;
mod control_input;
mod game_audio;
mod markers;
mod menu;
mod moveable;
mod setup;

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub(crate) enum GameState {
    Game,
    #[default]
    Menu,
}

fn main() {
    // Minimal app with single window and non-default title
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Naughty And Nice".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(
                    // This sets image filtering to nearest
                    // This is done to prevent textures with low resolution (e.g. pixel art) from being blurred
                    // by linear filtering.
                    ImagePlugin::default_nearest(),
                ),
            InitialSetup,
        ))
        // Declare the game state, whose starting value is determined by the `Default` trait
        .add_state::<GameState>()
        .add_plugins(menu::MenuPlugin)
        .run();
}
