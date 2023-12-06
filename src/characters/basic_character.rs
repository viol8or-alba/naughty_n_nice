use bevy::ecs::bundle::Bundle;

use crate::{animation::Animated, markers::CharacterMarker, moveable::Moveable};

/// This bundle defines a basic moveable, animated character.
#[derive(Bundle)]
pub(crate) struct BasicCharacter {
    pub(crate) animated: Animated,
    pub(crate) character_marker: CharacterMarker,
    pub(crate) moveable: Moveable,
}
