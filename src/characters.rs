mod basic_character;
mod character_with_status;
mod status;

pub(crate) use basic_character::BasicCharacter;
pub(crate) use character_with_status::CharacterWithStatus;
pub(crate) use status::Status;

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
