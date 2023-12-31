use bevy::ecs::component::Component;

use super::CharacterState;

/// Used to store the current [`CharacterState`] and health of a character.
#[derive(Component)]
pub(crate) struct Status {
    state: CharacterState,
    pub(crate) health: u8,
    pub(crate) game_over: bool,
}

impl Status {
    /// Creates a new [`Status`] with the given health.
    pub(crate) fn new(health: u8) -> Self {
        Self {
            state: CharacterState::Alive,
            health,
            game_over: false,
        }
    }

    /// Returns the current [`Status`] of the character.
    pub(crate) fn state(&self) -> CharacterState {
        self.state
    }

    /// Remove the given number of health points.
    pub(crate) fn remove_health(&mut self, to_remove: u8) {
        // Prevent overflows of our unsigned health value:
        // we want to remove more health than we have, we're
        // dead.
        if self.health < to_remove {
            self.state = CharacterState::Dead;
            return;
        }

        self.health -= to_remove;
        println!("Health: {}", self.health);

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
