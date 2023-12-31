#![allow(clippy::type_complexity)]

use bevy::{prelude::*, window::WindowResolution};
use setup::InitialSetup;

mod animation;
mod characters;
mod collision;
mod control_input;
mod game_audio;
mod markers;
mod menu;
mod moveable;
mod present;
mod setup;

pub(crate) const WINDOW_WIDTH: f32 = 480.;
pub(crate) const WINDOW_HEIGHT: f32 = 288.;

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub(crate) enum GameState {
    Game,
    #[default]
    Menu,
    Win,
}

fn main() {
    // Minimal app with single window and non-default title
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Naughty And Nice".to_string(),
                        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                        resizable: false,
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
