use bevy::{ecs::{system::{Res, Query}, query::With}, input::{Input, keyboard::KeyCode}, audio::AudioSink};
use crate::game_audio::Audio;
use bevy::prelude::AudioSinkPlayback;

pub(crate) fn mute(keyboard_input: Res<Input<KeyCode>>, music_controller: Query<&AudioSink, With<Audio>>) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if let Ok(sink) = music_controller.get_single() {
            sink.toggle();
        }
    }
}
