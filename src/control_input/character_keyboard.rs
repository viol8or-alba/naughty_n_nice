use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    input::{keyboard::KeyCode, Input},
};

use crate::{characters::Direction, markers::CharacterMarker};

pub(crate) fn handle_keyboard_for_character(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Direction, With<CharacterMarker>>,
) {
    query.for_each_mut(|mut direction| {
        if keyboard_input.any_just_released([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D]) {
            *direction = Direction::Static;
        }

        if keyboard_input.just_pressed(KeyCode::W) {
            *direction = Direction::Forward;
        } else if keyboard_input.just_pressed(KeyCode::A) {
            *direction = Direction::Left;
        } else if keyboard_input.just_pressed(KeyCode::S) {
            *direction = Direction::Back;
        } else if keyboard_input.just_pressed(KeyCode::D) {
            *direction = Direction::Right;
        }
    });
}
