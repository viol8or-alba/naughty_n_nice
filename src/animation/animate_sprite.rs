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

use crate::{
    characters::{CharacterState, Direction, Status},
    markers::CharacterMarker,
    moveable::Moveable,
};

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
            &Status,
        ),
        With<CharacterMarker>,
    >,
) {
    for (transform, indices, mut timer, mut sprite, mut ping_pong, moveable, status) in &mut query {
        let delta = time.delta();
        let delta_seconds = delta.as_secs_f32();
        timer.0.tick(delta);
        if timer.0.just_finished() {
            match status.state {
                CharacterState::Alive => handle_status_alive(
                    sprite,
                    ping_pong,
                    moveable,
                    indices,
                    transform,
                    delta_seconds,
                ),
                CharacterState::Celebrating => {
                    (sprite.index, *ping_pong) = determine_frame_oneshot(
                        indices.celebrate_start..=indices.celebrate_end,
                        &sprite.index,
                    )
                }
                CharacterState::Dead => {
                    (sprite.index, *ping_pong) = determine_frame_oneshot(
                        indices.die_start..=indices.die_end,
                        &sprite.index,
                    )
                }
            }
        }
    }
}

fn handle_status_alive(
    mut sprite: bevy::prelude::Mut<'_, TextureAtlasSprite>,
    mut ping_pong: bevy::prelude::Mut<'_, PingPong>,
    moveable: &Moveable,
    indices: &AnimationIndices,
    mut transform: bevy::prelude::Mut<'_, Transform>,
    delta_seconds: f32,
) {
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

fn handle_back(
    sprite_index: usize,
    ping_pong: &PingPong,
    indices: &AnimationIndices,
    transform: &mut Transform,
    delta_seconds: f32,
) -> (usize, PingPong) {
    transform.translation.y -= 150. * delta_seconds;
    determine_frame_moving(
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
    determine_frame_moving(
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
    determine_frame_moving(
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
    determine_frame_moving(
        indices.right_start..=indices.right_end,
        &sprite_index,
        ping_pong,
    )
}

fn determine_frame_moving(
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

fn determine_frame_oneshot(
    animation_range: RangeInclusive<usize>,
    sprite_index: &usize,
) -> (usize, PingPong) {
    // Have we started running this oneshot sequence?
    if !animation_range.contains(sprite_index) {
        // Start the sequence
        return (*animation_range.start(), PingPong::Ping);
    }

    // Reached last frame, stop animation by returning last frame again
    if sprite_index == animation_range.end() {
        return (*animation_range.end(), PingPong::Ping);
    }

    // Move to next frame
    (sprite_index + 1, PingPong::Ping)
}
