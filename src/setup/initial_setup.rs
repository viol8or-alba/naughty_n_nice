use crate::animation::{AnimateSprite, Animated, AnimationIndices, AnimationTimer, PingPong};
use crate::characters::{BasicCharacter, CharacterWithStatus, Direction, Inventory, Status};
use crate::collision::CollisionHandler;
use crate::control_input::ControlInput;
use crate::game_audio::Audio;
use crate::markers::{CameraMarker, CharacterMarker};
use crate::moveable::{Moveable, Speed};
use crate::present::Present;
use bevy::prelude::*;

use bevy_ecs_ldtk::{
    LdtkPlugin, LdtkSettings, LdtkWorldBundle, LevelSelection, LevelSpawnBehavior,
};

/* Constants */

pub(crate) const WALL_THICKNESS: f32 = 10.0;

// x coordinates
pub(crate) const LEFT_WALL: f32 = -450.;
pub(crate) const RIGHT_WALL: f32 = 450.;
// y coordinates
pub(crate) const BOTTOM_WALL: f32 = -300.;
pub(crate) const TOP_WALL: f32 = 300.;

const SCOREBOARD_FONT_SIZE: f32 = 20.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(10.0);

const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const TEXT_COLOR: Color = Color::rgb(0.2, 0.2, 0.9);
const SCORE_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

fn update_stats(
    player_query: Query<(&Status, &Inventory), With<CharacterMarker>>,
    mut query: Query<&mut Text, With<Scoreboard>>,
) {
    if let Ok((_status, inventory)) = player_query.get_single() {
        let mut text = query.single_mut();
        text.sections[1].value = inventory.number_of_presents().to_string();
    }
}

/// Plugin to set up initial scene with camera, player, and audio. Adds plugins
/// for sprite animation and handling keyboard control of sprite.
pub(crate) struct InitialSetup;

impl Plugin for InitialSetup {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene)
            .add_systems(Startup, setup_player)
            .add_systems(Startup, setup_presents)
            .add_systems(Startup, setup_audio)
            .add_systems(Startup, setup_walls)
            .add_systems(Startup, setup_scoreboard)
            .insert_resource(ClearColor(BACKGROUND_COLOR))
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: false,
                },
                ..Default::default()
            })
            .insert_resource(LevelSelection::Index(0))
            .add_plugins(LdtkPlugin)
            .add_plugins(AnimateSprite)
            .add_plugins(ControlInput)
            .add_plugins(CollisionHandler)
            .add_systems(Update, (bevy::window::close_on_esc, update_stats));
    }
}

/// Add the default 2D camera bundle.
fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), CameraMarker));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/ldtk/Naughty_n_Nice.ldtk"),
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    });
}

/// Setup the player. This will load up the player's sprite sheet and create a
/// texture atlas from it. The sprite sheet has three frames of animation for
/// each of the four movement directions. Each frame is displayed at 300ms intervals.
fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Load the sprite sheet
    let texture_handle = asset_server.load("sprites/deer.png");

    // Create the texture atlas: the sprite sheet is four rows with three animation frames
    // per row. Each frame is a 48x48 bitmap.
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(48.0, 48.0), 3, 6, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Set the start and end frames for each sprite animation.
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

    // Spawn the character in the scene. Character will start facing the camera.
    commands.spawn(CharacterWithStatus {
        basic_character: BasicCharacter {
            animated: Animated {
                sprite_sheet_bundle: SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(animation_indices.back_start + 1),
                    transform: Transform::from_xyz(0., 0., 10.),
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
        status: Status::new(100),
        inventory: Inventory::new(),
    });
}

/// Randomly spawn presents.
fn setup_presents(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/Gifts_Green.png"),
            transform: Transform::from_xyz(100., 100., 5.),
            ..Default::default()
        },
        Present::new(crate::present::PresentType::Nice),
    ));
}

/// Load the background audio into the asset server.
fn setup_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("sounds/Windless Slopes.ogg"),
            ..default()
        },
        Audio,
    ));
}

fn setup_walls(mut commands: Commands) {
    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
}

#[derive(Component)]
struct Scoreboard;

fn setup_scoreboard(mut commands: Commands) {
    // Scoreboard
    commands.spawn(
        (TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        }), Scoreboard),
    );
}

#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;

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
