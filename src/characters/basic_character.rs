use bevy::{ecs::bundle::Bundle, sprite::SpriteSheetBundle};

use crate::{
    animation::{AnimationIndices, AnimationTimer},
    markers::CharacterMarker,
};

use super::Direction;

#[derive(Bundle, Clone)]
pub(crate) struct BasicCharacter {
    pub(crate) sprite_sheet_bundle: SpriteSheetBundle,
    pub(crate) animation_indices: AnimationIndices,
    pub(crate) animation_timer: AnimationTimer,
    pub(crate) character_marker: CharacterMarker,
    pub(crate) direction: Direction,
}
