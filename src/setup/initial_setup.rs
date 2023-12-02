use bevy::prelude::*;
use crate::characters::BasicCharacter;
use crate::markers::CameraMarker;
use crate::animation::{Indices, AnimationTimer};

pub(crate) struct InitialSetup;

impl Plugin for InitialSetup {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_player)
        .add_systems(Update, animate_sprite);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        CameraMarker,
    ));
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>){    
    let texture_handle = asset_server.load("sprites/flamingo.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(48.0, 48.0), 3, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = Indices{ foward_start: 0, foward_end: 2, back_start: 9, back_end: 11, left_start: 3, left_end: 5, right_start: 6, right_end: 8 };
    commands.spawn(
        BasicCharacter{
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.foward_start + 1),
                // transform: Transform::from_scale(Vec3::splat(6.0)),
                ..default()
            },
            animation_indices,
            animation_timer: AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            character_marker: crate::markers::CharacterMarker,
        }
    );
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &Indices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            sprite.index = if sprite.index == indices.foward_end {
                indices.foward_start
            } else {
                sprite.index + 1
            };
        }
    }
}
