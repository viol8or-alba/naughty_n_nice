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

/// A character can be in one of the following state:
/// - `Alive` and able to move
/// - `Celebrating` completed the level
/// - `Dead` failed to complete the level
#[derive(Clone, Copy)]
pub(crate) enum CharacterState {
    Alive,
    Celebrating,
    Dead,
}
