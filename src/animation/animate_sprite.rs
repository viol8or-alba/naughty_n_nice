use std::ops::RangeInclusive;

use bevy::{
    app::{Plugin, Update},
    ecs::{
        query::With,
        system::{Query, Res},
    },
    sprite::TextureAtlasSprite,
    time::Time,
    transform::components::Transform,
};

use crate::{characters::Direction, markers::CharacterMarker};

use super::{AnimationIndices, AnimationTimer};

pub(crate) struct AnimateSprite;

impl Plugin for AnimateSprite {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, animate_sprite);
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<
        (
            &mut Transform,
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Direction,
        ),
        With<CharacterMarker>,
    >,
) {
    for (mut transform, indices, mut timer, mut sprite, direction) in &mut query {
        let delta = time.delta();
        let delta_seconds = delta.as_secs_f32();
        timer.0.tick(delta);
        if timer.0.just_finished() {
            sprite.index = match direction {
                Direction::Static => indices.back_start + 1,
                Direction::Back => handle_back(sprite.index, indices, &mut transform, delta_seconds),
                Direction::Forward => handle_forward(sprite.index, indices, &mut transform, delta_seconds),
                Direction::Left => handle_left(sprite.index, indices, &mut transform, delta_seconds),
                Direction::Right => handle_right(sprite.index, indices, &mut transform, delta_seconds),
            }
        }
    }
}

fn handle_back(sprite_index: usize, indices: &AnimationIndices, transform: &mut Transform, delta_seconds: f32) -> usize {
    transform.translation.y -= 150. * delta_seconds;
    determine_frame(indices.back_start..=indices.back_end, &sprite_index)
}

fn handle_forward(sprite_index: usize, indices: &AnimationIndices, transform: &mut Transform, delta_seconds: f32) -> usize {
    transform.translation.y += 150. * delta_seconds;
    determine_frame(indices.forward_start..=indices.forward_end, &sprite_index)
}

fn handle_left(sprite_index: usize, indices: &AnimationIndices, transform: &mut Transform, delta_seconds: f32) -> usize {
    transform.translation.x -= 150. * delta_seconds;
    determine_frame(indices.left_start..=indices.left_end, &sprite_index)
}

fn handle_right(sprite_index: usize, indices: &AnimationIndices, transform: &mut Transform, delta_seconds: f32) -> usize {
    transform.translation.x += 150. * delta_seconds;
    determine_frame(indices.right_start..=indices.right_end, &sprite_index)
}

fn determine_frame(animation_range: RangeInclusive<usize>, sprite_index: &usize) -> usize {
    // Were we previously moving in the current direction?
    if !animation_range.contains(sprite_index) {
        // Start moving in the current direction
        return *animation_range.start();
    }

    // Reached last frame, wrap round to first frame
    if sprite_index == animation_range.end() {
        return *animation_range.start();
    }

    // Move to next frame
    sprite_index + 1
}
