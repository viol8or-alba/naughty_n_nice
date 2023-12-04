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

use crate::{characters::Direction, markers::CharacterMarker, moveable::Moveable};

use super::{AnimationIndices, AnimationTimer, PingPong};

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
            &mut PingPong,
            &Moveable,
        ),
        With<CharacterMarker>,
    >,
) {
    for (mut transform, indices, mut timer, mut sprite, mut ping_pong, moveable) in &mut query {
        let delta = time.delta();
        let delta_seconds = delta.as_secs_f32();
        timer.0.tick(delta);
        if timer.0.just_finished() {
            (sprite.index, *ping_pong) = match moveable.direction {
                Direction::Static => (indices.back_start + 1, PingPong::Ping),
                Direction::Back => handle_back(
                    sprite.index,
                    &ping_pong,
                    indices,
                    &mut transform,
                    delta_seconds,
                ),
                Direction::Forward => handle_forward(
                    sprite.index,
                    &ping_pong,
                    indices,
                    &mut transform,
                    delta_seconds,
                ),
                Direction::Left => handle_left(
                    sprite.index,
                    &ping_pong,
                    indices,
                    &mut transform,
                    delta_seconds,
                ),
                Direction::Right => handle_right(
                    sprite.index,
                    &ping_pong,
                    indices,
                    &mut transform,
                    delta_seconds,
                ),
            }
        }
    }
}

fn handle_back(
    sprite_index: usize,
    ping_pong: &PingPong,
    indices: &AnimationIndices,
    transform: &mut Transform,
    delta_seconds: f32,
) -> (usize, PingPong) {
    transform.translation.y -= 150. * delta_seconds;
    determine_frame(
        indices.back_start..=indices.back_end,
        &sprite_index,
        ping_pong,
    )
}

fn handle_forward(
    sprite_index: usize,
    ping_pong: &PingPong,
    indices: &AnimationIndices,
    transform: &mut Transform,
    delta_seconds: f32,
) -> (usize, PingPong) {
    transform.translation.y += 150. * delta_seconds;
    determine_frame(
        indices.forward_start..=indices.forward_end,
        &sprite_index,
        ping_pong,
    )
}

fn handle_left(
    sprite_index: usize,
    ping_pong: &PingPong,
    indices: &AnimationIndices,
    transform: &mut Transform,
    delta_seconds: f32,
) -> (usize, PingPong) {
    transform.translation.x -= 150. * delta_seconds;
    determine_frame(
        indices.left_start..=indices.left_end,
        &sprite_index,
        ping_pong,
    )
}

fn handle_right(
    sprite_index: usize,
    ping_pong: &PingPong,
    indices: &AnimationIndices,
    transform: &mut Transform,
    delta_seconds: f32,
) -> (usize, PingPong) {
    transform.translation.x += 150. * delta_seconds;
    determine_frame(
        indices.right_start..=indices.right_end,
        &sprite_index,
        ping_pong,
    )
}

fn determine_frame(
    animation_range: RangeInclusive<usize>,
    sprite_index: &usize,
    ping_pong: &PingPong,
) -> (usize, PingPong) {
    // Were we previously facing in the current direction?
    if !animation_range.contains(sprite_index) {
        // Face in the current direction
        return (*animation_range.start(), PingPong::Ping);
    }

    match ping_pong {
        PingPong::Ping => {
            // Reached last frame, reverse animation
            if sprite_index == animation_range.end() {
                return (*animation_range.end() - 1, PingPong::Pong);
            }
        }
        PingPong::Pong => {
            // Reached first frame, forward animation
            if sprite_index == animation_range.start() {
                return (*animation_range.start() + 1, PingPong::Ping);
            }
        }
    }

    // Move to next frame
    match ping_pong {
        PingPong::Ping => (sprite_index + 1, PingPong::Ping),
        PingPong::Pong => (sprite_index - 1, PingPong::Ping),
    }
}
