mod animate_sprite;
mod indices;
mod timer;

pub(crate) use animate_sprite::AnimateSprite;
use bevy::{
    ecs::{bundle::Bundle, component::Component},
    sprite::SpriteSheetBundle,
};
pub(crate) use indices::AnimationIndices;
pub(crate) use timer::AnimationTimer;

/// The animation is defined with three frames. For example when walking
/// left we have left leg forward, both legs together, right leg forward.
/// To get a natural looking walk, we need to reverse that animation 
/// sequence after right leg forward. This flag will indicate whether the
/// animation is running forwards (Ping) or backward (Pong).
#[derive(Component, Clone, Copy)]
pub(crate) enum PingPong {
    Ping,
    Pong,
}

/// This bundle defines an entity that is animated. It bundles the following
/// components:
/// - The sprite sheet
/// - The animation indices for each of forwards, backwards, left and right
/// - The animation timer
/// - A flag that determines whether the animation is played from start to
/// end or from end to start
#[derive(Bundle, Clone)]
pub(crate) struct Animated {
    pub(crate) sprite_sheet_bundle: SpriteSheetBundle,
    pub(crate) animation_indices: AnimationIndices,
    pub(crate) animation_timer: AnimationTimer,
    pub(crate) ping_pong: PingPong,
}
