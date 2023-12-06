mod basic_character;
mod character_with_status;
mod status;

pub(crate) use basic_character::BasicCharacter;
pub(crate) use character_with_status::CharacterWithStatus;
pub(crate) use status::Status;

/// Used to store current movement direction of a sprite.
/// Also used to determine which animation frames are used
/// to draw the sprite.
#[derive(Clone, Copy)]
pub(crate) enum Direction {
    Static,
    Back,
    Forward,
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub(crate) enum CharacterState {
    Alive,
    Celebrating,
    Dead,
}
