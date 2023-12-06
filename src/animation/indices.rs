use bevy::ecs::component::Component;

/// Stores the indices within the sprite sheet for the start and end
/// frames of each of moving forward, backward, left and right.
/// Also contains indices for celebration and death animations.
#[derive(Component, Clone)]
pub(crate) struct AnimationIndices {
    pub(crate) forward_start: usize,
    pub(crate) forward_end: usize,
    pub(crate) back_start: usize,
    pub(crate) back_end: usize,
    pub(crate) left_start: usize,
    pub(crate) left_end: usize,
    pub(crate) right_start: usize,
    pub(crate) right_end: usize,
    pub(crate) celebrate_start: usize,
    pub(crate) celebrate_end: usize,
    pub(crate) die_start: usize,
    pub(crate) die_end: usize,
}
