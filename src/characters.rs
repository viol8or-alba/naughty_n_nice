mod basic_character;

pub(crate) use basic_character::BasicCharacter;

#[derive(Clone, Copy)]
pub(crate) enum Direction {
    Static,
    Back,
    Forward,
    Left,
    Right,
}
