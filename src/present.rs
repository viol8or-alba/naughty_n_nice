use bevy::ecs::component::Component;

#[derive(PartialEq, Eq, Clone, Copy)]
pub(crate) enum PresentType {
    // Naughty stores the number of hit points to remove
    Naughty(u8),
    Nice,
}

/// Component that defines a present of a given type:
/// - Naughty: this will remove health points
/// - Nice: this will count towards the collected present total
#[derive(Component)]
pub(crate) struct Present {
    present_type: PresentType,
}

impl Present {
    /// Creates a new [`Present`] of the given type.
    pub(crate) fn new(present_type: PresentType) -> Self {
        Self { present_type }
    }

    /// Returns the [`PresentType`] of this [`Present`].
    pub(crate) fn present_type(&self) -> PresentType {
        self.present_type
    }
}
