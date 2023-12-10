use bevy::ecs::component::Component;

/// Component that tracks the player's inventory.
#[derive(Component)]
pub(crate) struct Inventory {
    number_of_presents: u8,
}

impl Inventory {
    /// Creates a new empty [`Inventory`].
    pub(crate) fn new() -> Self {
        Self {
            number_of_presents: 0,
        }
    }

    /// Add a present to this [`Inventory`].
    pub(crate) fn add_present(&mut self) {
        self.number_of_presents += 1;
        println!("Presents: {}", self.number_of_presents);
    }

    /// Returns the number of presents in this [`Inventory`].
    pub(crate) fn number_of_presents(&self) -> u8 {
        self.number_of_presents
    }
}
