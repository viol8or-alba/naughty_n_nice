use crate::game_audio::Audio;
use bevy::prelude::AudioSinkPlayback;
use bevy::{
    audio::AudioSink,
    ecs::{
        query::With,
        system::{Query, Res},
    },
    input::{keyboard::KeyCode, Input},
};

/// Handle toggling the background music on and off.
pub(crate) fn mute(
    keyboard_input: Res<Input<KeyCode>>,
    music_controller: Query<&AudioSink, With<Audio>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if let Ok(sink) = music_controller.get_single() {
            sink.toggle();
        }
    }
}
