use crate::animation::{AnimateSprite, Animated, AnimationIndices, AnimationTimer, PingPong};
use crate::characters::{BasicCharacter, Direction};
use crate::control_input::ControlInput;
use crate::markers::CameraMarker;
use crate::moveable::{Moveable, Speed};
use crate::game_audio::Audio;
use bevy::prelude::*;

pub(crate) struct InitialSetup;

impl Plugin for InitialSetup {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Startup, setup_player)
            .add_systems(Startup, setup_audio)
            .add_plugins(AnimateSprite)
            .add_plugins(ControlInput);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), CameraMarker));
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/flamingo.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(48.0, 48.0), 3, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices {
        forward_start: 9,
        forward_end: 11,
        back_start: 0,
        back_end: 2,
        left_start: 3,
        left_end: 5,
        right_start: 6,
        right_end: 8,
    };
    commands.spawn(BasicCharacter {
        animated: Animated {
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.back_start + 1),
                // transform: Transform::from_scale(Vec3::splat(6.0)),
                ..default()
            },
            animation_indices,
            animation_timer: AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            ping_pong: PingPong::Ping,
        },
        character_marker: crate::markers::CharacterMarker,
        moveable: Moveable {
            direction: Direction::Static,
            speed: Speed(1.0),
        },
    });
}

fn setup_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((AudioBundle {
        source: asset_server.load("sounds/Windless Slopes.ogg"),
        ..default()
    }, Audio));
}

