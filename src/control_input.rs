use bevy::app::{Plugin, Update};

mod audio_keyboard;
mod character_keyboard;

use audio_keyboard::mute;
use character_keyboard::handle_keyboard_for_character;

/// This plugin handles keyboard input.
pub(crate) struct ControlInput;

impl Plugin for ControlInput {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (handle_keyboard_for_character, mute));
    }
}
