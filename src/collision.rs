use bevy::{
    app::{Plugin, Update},
    ecs::{query::With, system::{Query, Commands}, entity::Entity},
    math::Vec2,
    sprite::collide_aabb::collide,
    transform::components::Transform,
};

use crate::{
    characters::Status,
    markers::CharacterMarker,
    present::{Present, PresentType},
};

/// Check player character for collisions with other entities
pub(crate) struct CollisionHandler;

impl Plugin for CollisionHandler {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, hit_test_presents);
    }
}

/// Check for collisions with presents and update player state depending on the
/// type of present.
fn hit_test_presents(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Status), With<CharacterMarker>>,
    mut present_query: Query<(Entity, &Transform, &Present)>,
) {
    // Both the presents and the player are the same size
    let entity_size = Vec2::new(48., 48.);

    // Get the sigle player entity
    match player_query.get_single_mut() {
        Ok((player_transform, mut status)) => {
            // Loop through the presents and test each one against the player
            for (entity, present_transform, present) in &mut present_query {
                if collide(
                    player_transform.translation,
                    entity_size,
                    present_transform.translation,
                    entity_size,
                )
                .is_some()
                {
                    if present.present_type() == PresentType::Naughty {
                        status.remove_health(100);
                    }

                    // Despawn the present, we're done with it
                    commands.entity(entity).despawn();
                }
            }
        }
        Err(_) => println!("Either player is missing or there is more than one player"),
    }
}
