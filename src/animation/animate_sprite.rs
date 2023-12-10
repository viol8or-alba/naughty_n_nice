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
    setup::{
        CHARACTER_BOTTOM_BOUND, CHARACTER_LEFT_BOUND, CHARACTER_RIGHT_BOUND, CHARACTER_TOP_BOUND,
    },
};

use super::{AnimationIndices, AnimationTimer, PingPong};

/// Defines a plugin used to animate and move the sprite based on its current
/// [`Direction`].
pub(crate) struct AnimateSprite;

impl Plugin for AnimateSprite {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, animate_sprite);
    }
}

/// Query for a sprite with the [`CharacterMarker`] component. We want:
/// - its transform (to set its position in the scene)
/// - animation indices (to determine the current frame)
/// - the animation timer (is it time to display the next frame/move)
/// - the actual sprite to animate
/// - the animation sequence (forward or back)
/// - the currently set direction (from keyboard input, handled by the
/// [`crate::control_input::ControlInput`] plugin)
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
            &mut Status,
        ),
        With<CharacterMarker>,
    >,
) {
    for (transform, indices, mut timer, mut sprite, mut ping_pong, moveable, mut status) in
        &mut query
    {
        let delta = time.delta();
        let delta_seconds = delta.as_secs_f32();
        timer.0.tick(delta);

        // Time for the next frame?
        if timer.0.just_finished() {
            match status.state() {
                CharacterState::Alive => handle_status_alive(
                    sprite,
                    ping_pong,
                    moveable,
                    indices,
                    transform,
                    delta_seconds,
                ),
                CharacterState::Celebrating => {
                    // Run celebraion animation once
                    (sprite.index, *ping_pong) = determine_frame_oneshot(
                        indices.celebrate_start..=indices.celebrate_end,
                        &sprite.index,
                    );

                    if sprite.index == indices.celebrate_end {
                        status.end_celebration();
                    }
                }
                CharacterState::Dead => {
                    // Run death animation once
                    (sprite.index, *ping_pong) =
                        determine_frame_oneshot(indices.die_start..=indices.die_end, &sprite.index)
                }
            }
        }
    }
}

/// Handle movement of a character that has neither completed the level (`Celebrating`)
/// or failed to complete the level (`Dead`).
fn handle_status_alive(
    mut sprite: bevy::prelude::Mut<'_, TextureAtlasSprite>,
    mut ping_pong: bevy::prelude::Mut<'_, PingPong>,
    moveable: &Moveable,
    indices: &AnimationIndices,
    mut transform: bevy::prelude::Mut<'_, Transform>,
    delta_seconds: f32,
) {
    // Which direction are we moving in
    (sprite.index, *ping_pong) = match moveable.direction {
        // Not moving, draw the sprite facing the camera
        Direction::Static => (indices.back_start + 1, PingPong::Ping),

        // We are moving
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

/// Handle moving towards the camera: update the transform and
/// determine the next animation frame to draw.
fn handle_back(
    sprite_index: usize,
    ping_pong: &PingPong,
    indices: &AnimationIndices,
    transform: &mut Transform,
    delta_seconds: f32,
) -> (usize, PingPong) {
    let new_position = transform.translation.y - 300. * delta_seconds;

    transform.translation.y = new_position.clamp(CHARACTER_BOTTOM_BOUND, CHARACTER_TOP_BOUND);

    determine_frame_moving(
        indices.back_start..=indices.back_end,
        &sprite_index,
        ping_pong,
    )
}

/// Handle moving away from the camera: update the transform and
/// determine the next animation frame to draw.
fn handle_forward(
    sprite_index: usize,
    ping_pong: &PingPong,
    indices: &AnimationIndices,
    transform: &mut Transform,
    delta_seconds: f32,
) -> (usize, PingPong) {
    let new_position = transform.translation.y + 300. * delta_seconds;

    transform.translation.y = new_position.clamp(CHARACTER_BOTTOM_BOUND, CHARACTER_TOP_BOUND);

    determine_frame_moving(
        indices.forward_start..=indices.forward_end,
        &sprite_index,
        ping_pong,
    )
}

/// Handle moving left: update the transform and determine the next
/// animation frame to draw.
fn handle_left(
    sprite_index: usize,
    ping_pong: &PingPong,
    indices: &AnimationIndices,
    transform: &mut Transform,
    delta_seconds: f32,
) -> (usize, PingPong) {
    let new_position = transform.translation.x - 300. * delta_seconds;
    transform.translation.x = new_position.clamp(CHARACTER_LEFT_BOUND, CHARACTER_RIGHT_BOUND);

    determine_frame_moving(
        indices.left_start..=indices.left_end,
        &sprite_index,
        ping_pong,
    )
}

/// Handle moving left: update the transform and determine the next
/// animation frame to draw.
fn handle_right(
    sprite_index: usize,
    ping_pong: &PingPong,
    indices: &AnimationIndices,
    transform: &mut Transform,
    delta_seconds: f32,
) -> (usize, PingPong) {
    let new_position = transform.translation.x + 300. * delta_seconds;

    transform.translation.x = new_position.clamp(CHARACTER_LEFT_BOUND, CHARACTER_RIGHT_BOUND);

    determine_frame_moving(
        indices.right_start..=indices.right_end,
        &sprite_index,
        ping_pong,
    )
}

/// Based on the given range of frames, current frame index
/// and animation direction, determine the next frame to
/// display.
/// Return the index of the next frame and whether we are playing
/// the animation forward or backward.
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

    // Determine the animation frame sequence
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

/// For a one-shot animation, determine which frame in the given
/// range to display next based on the given index.
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
