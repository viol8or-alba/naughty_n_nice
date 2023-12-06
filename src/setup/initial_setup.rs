use crate::animation::{AnimateSprite, Animated, AnimationIndices, AnimationTimer, PingPong};
use crate::characters::{BasicCharacter, CharacterState, CharacterWithStatus, Direction, Status};
use crate::control_input::ControlInput;
use crate::game_audio::Audio;
use crate::markers::CameraMarker;
use crate::moveable::{Moveable, Speed};
use bevy::prelude::*;

/* Constants */

const WALL_THICKNESS: f32 = 10.0;

// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub(crate) struct InitialSetup;

impl Plugin for InitialSetup {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Startup, setup_player)
            .add_systems(Startup, setup_audio)
            .add_systems(Startup, setup_walls)
            .insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_plugins(AnimateSprite)
            .add_plugins(ControlInput)
            .add_systems(Update, bevy::window::close_on_esc);
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
    let texture_handle = asset_server.load("sprites/deer.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(48.0, 48.0), 3, 6, None, None);
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
        celebrate_start: 12,
        celebrate_end: 14,
        die_start: 15,
        die_end: 17,
    };
    commands.spawn(CharacterWithStatus {
        basic_character: BasicCharacter {
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
        },
        status: Status {
            state: CharacterState::Alive,
        },
    });
}

fn setup_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("sounds/Windless Slopes.ogg"),
            ..default()
        },
        Audio,
    ));
}

fn setup_walls(asset_server: Res<AssetServer>, mut commands: Commands) {
        // Walls
        commands.spawn(WallBundle::new(WallLocation::Left));
        commands.spawn(WallBundle::new(WallLocation::Right));
        commands.spawn(WallBundle::new(WallLocation::Bottom));
        commands.spawn(WallBundle::new(WallLocation::Top));
   
}

#[derive(Component)]
struct Collider;

/// Which side of the arena is this wall located on?
enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: location.position().extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

