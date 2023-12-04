use bevy::app::{Plugin, Update};

mod character_keyboard;
mod audio_keyboard;

use character_keyboard::handle_keyboard_for_character;
use audio_keyboard::mute;

pub(crate) struct ControlInput;

impl Plugin for ControlInput {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (handle_keyboard_for_character, mute));
    }
}
