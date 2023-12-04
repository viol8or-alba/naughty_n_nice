use bevy::ecs::bundle::Bundle;

use crate::{animation::Animated, markers::CharacterMarker, moveable::Moveable};

#[derive(Bundle, Clone)]
pub(crate) struct BasicCharacter {
    pub(crate) animated: Animated,
    pub(crate) character_marker: CharacterMarker,
    pub(crate) moveable: Moveable,
}
