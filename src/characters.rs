use bevy::{ecs::bundle::Bundle, sprite::SpriteSheetBundle};

use crate::{animation::{Indices, AnimationTimer}, markers::CharacterMarker};

#[derive(Bundle, Clone)]
pub(crate) struct BasicCharacter{
    pub(crate) sprite_sheet_bundle: SpriteSheetBundle,
    pub(crate) animation_indices: Indices,
    pub(crate) animation_timer: AnimationTimer,
    pub(crate) character_marker: CharacterMarker,
}