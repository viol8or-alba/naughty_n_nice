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

#[derive(Component, Clone, Copy)]
pub(crate) enum PingPong {
    Ping,
    Pong,
}

#[derive(Bundle, Clone)]
pub(crate) struct Animated {
    pub(crate) sprite_sheet_bundle: SpriteSheetBundle,
    pub(crate) animation_indices: AnimationIndices,
    pub(crate) animation_timer: AnimationTimer,
    pub(crate) ping_pong: PingPong,
}
