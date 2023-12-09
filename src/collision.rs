use bevy::{
    app::{Plugin, Update},
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, Query},
    },
    math::Vec2,
    sprite::collide_aabb::collide,
    transform::components::Transform,
};

use crate::{
    characters::{Inventory, Status},
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
    mut player_query: Query<(&Transform, &mut Status, &mut Inventory), With<CharacterMarker>>,
    mut present_query: Query<(Entity, &Transform, &Present)>,
) {
    // Both the presents and the player are the same size
    let entity_size = Vec2::new(48., 48.);

    // Get the single player entity
    match player_query.get_single_mut() {
        Ok((player_transform, mut status, mut inventory)) => {
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
                    match present.present_type() {
                        PresentType::Naughty(health_to_remove) => {
                            status.remove_health(health_to_remove)
                        }
                        PresentType::Nice => {
                            // Add present to inventory and check for win condition
                            inventory.add_present();
                            check_and_update_win_condition(&inventory, &mut status)
                        }
                    }

                    // Despawn the present, we're done with it
                    commands.entity(entity).despawn();
                }
            }
        }
        Err(_) => println!("Either player is missing or there is more than one player"),
    }
}

/// Compare the number of presents in the inventory with the win condition and
/// set the player celebrating if the win condition is met.
fn check_and_update_win_condition(inventory: &Inventory, status: &mut Status) {
    if inventory.number_of_presents() == 1 {
        status.celebrate();
    }
}
