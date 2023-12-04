use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    input::{keyboard::KeyCode, Input},
};

use crate::{characters::Direction, markers::CharacterMarker, moveable::Moveable};

pub(crate) fn handle_keyboard_for_character(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Moveable, With<CharacterMarker>>,
) {
    query.for_each_mut(|mut moveable| {
        if keyboard_input.any_just_released([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D]) {
            moveable.direction = Direction::Static;
        }

        if keyboard_input.pressed(KeyCode::W) {
            moveable.direction = Direction::Forward;
        } else if keyboard_input.pressed(KeyCode::A) {
            moveable.direction = Direction::Left;
        } else if keyboard_input.pressed(KeyCode::S) {
            moveable.direction = Direction::Back;
        } else if keyboard_input.pressed(KeyCode::D) {
            moveable.direction = Direction::Right;
        }
    });
}
