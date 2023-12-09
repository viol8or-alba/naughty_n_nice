use bevy::ecs::component::Component;

use super::CharacterState;

/// Used to store the current [`CharacterState`] and health of a character.
#[derive(Component)]
pub(crate) struct Status {
    state: CharacterState,
    health: u8,
}

impl Status {
    /// Creates a new [`Status`] with the given health.
    pub(crate) fn new(health: u8) -> Self {
        Self {
            state: CharacterState::Alive,
            health,
        }
    }

    /// Returns the current [`Status`] of the character.
    pub(crate) fn state(&self) -> CharacterState {
        self.state
    }

    /// Remove the given number of health points.
    pub(crate) fn remove_health(&mut self, to_remove: u8) {
        self.health -= to_remove;

        if self.health == 0 {
            self.state = CharacterState::Dead;
        }
    }

    /// Indicate that the character is celebrating.
    pub(crate) fn celebrate(&mut self) {
        self.state = CharacterState::Celebrating;
    }

    /// End the character celebration.
    pub(crate) fn end_celebration(&mut self) {
        if self.state == CharacterState::Celebrating {
            self.state = CharacterState::Alive;
        }
    }
}
