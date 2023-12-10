use std::ops::Range;

use crate::animation::{AnimateSprite, Animated, AnimationIndices, AnimationTimer, PingPong};
use crate::characters::{BasicCharacter, CharacterWithStatus, Direction, Inventory, Status};
use crate::collision::CollisionHandler;
use crate::control_input::ControlInput;
use crate::game_audio::Audio;
use crate::markers::{CameraMarker, CharacterMarker};
use crate::moveable::{Moveable, Speed};
use crate::present::{Present, PresentType};
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;

use bevy::utils::HashSet;
use bevy_ecs_ldtk::{
    LdtkPlugin, LdtkSettings, LdtkWorldBundle, LevelSelection, LevelSpawnBehavior,
};
use rand::Rng;

/* Constants */

pub(crate) const WALL_THICKNESS: f32 = 10.0;

// x coordinates
pub(crate) const LEFT_WALL: f32 = -450.;
pub(crate) const RIGHT_WALL: f32 = 450.;
// y coordinates
pub(crate) const BOTTOM_WALL: f32 = -300.;
pub(crate) const TOP_WALL: f32 = 300.;

const SCOREBOARD_FONT_SIZE: f32 = 20.0;
const SCORE_BASIC_TEXT_PADDING: Val = Val::Px(10.0);
const SCORE_NAUGHTY_TEXT_PADDING_LEFT: Val = Val::Px(WINDOW_WIDTH - 120.);

const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const TEXT_COLOR: Color = Color::rgb(0.2, 0.2, 0.9);
const RED_TEXT_COLOR: Color = Color::rgb(0.9, 0.2, 0.5);
const SCORE_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

// Coordinate range for spawning presents, make sure we don't spawn partially outside the screen
// or on the back wall.
pub(crate) const CHARACTER_TOP_BOUND: f32 = (WINDOW_HEIGHT / 2.) - 30. / 2.0 - 15.0; // TODO replace 5 with character height/2 + padding
pub(crate) const CHARACTER_BOTTOM_BOUND: f32 = -(WINDOW_HEIGHT / 2.) + WALL_THICKNESS / 2.0 + 18.0; // TODO replace 5 with character height/2 + padding
pub(crate) const CHARACTER_LEFT_BOUND: f32 = -(WINDOW_WIDTH / 2.) + WALL_THICKNESS / 2.0 + 8.0; // TODO replace 5 with character width/2 + padding
pub(crate) const CHARACTER_RIGHT_BOUND: f32 = (WINDOW_WIDTH / 2.) - WALL_THICKNESS / 2.0 - 8.0; // TODO replace 5 with character width/2 + padding

const X_RANGE: Range<i32> =
    ((CHARACTER_LEFT_BOUND / 10.) as i32)..((CHARACTER_RIGHT_BOUND / 10.) as i32);
const Y_RANGE: Range<i32> =
    ((CHARACTER_BOTTOM_BOUND / 10.) as i32)..((CHARACTER_TOP_BOUND / 10.) as i32);

fn update_stats(
    player_query: Query<(&Status, &Inventory), With<CharacterMarker>>,
    mut query_nice: Query<&mut Text, With<CounterNice>>,
    mut query_naughty: Query<&mut Text, (With<Health>, Without<CounterNice>)>,
) {
    if let Ok((status, inventory)) = player_query.get_single() {
        let mut text = query_nice.single_mut();
        text.sections[1].value = inventory.number_of_presents().to_string();

        let mut text = query_naughty.single_mut();
        text.sections[1].value = status.health.to_string();
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
            .add_systems(Update, (bevy::window::close_on_esc, update_stats, end_game_win, end_game_lose));
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
    let mut rng = rand::thread_rng();
    let red_present = "sprites/Gifts_Red.png".to_string();
    let green_present = "sprites/Gifts_Green.png".to_string();

    let mut locations_spawned = HashSet::new();
    // skip character spawn location
    locations_spawned.insert((0, 0));

    println!("{X_RANGE:?}");
    println!("{Y_RANGE:?}");
    for count in 0..10 {
        let (present_type, current_present_image) = if count < 5 {
            (PresentType::Naughty(20), &red_present)
        } else {
            (PresentType::Nice, &green_present)
        };

        // Range is set to a tenth of screen size and then multiplied up to cut down on clustering of presents
        let mut x = rng.gen_range(X_RANGE) * 10;
        let mut y = rng.gen_range(Y_RANGE) * 10;

        // loop until we get a unique location
        while !locations_spawned.insert((x as u32, y as u32)) {
            x = rng.gen_range(X_RANGE) * 10;
            y = rng.gen_range(Y_RANGE) * 10;
        }

        println!("Spawning present at {x:.5}, {y:.5}");

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(current_present_image),
                transform: Transform::from_xyz(x as f32, y as f32, 5.),
                ..Default::default()
            },
            Present::new(present_type),
        ));
    }
}

/// Load the background audio into the asset server.
fn setup_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("sounds/Naughty_n_Nice.ogg"),
            settings: PlaybackSettings {
                mode: bevy_audio::PlaybackMode::Loop,
                ..Default::default()
            },
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
struct CounterNice;

#[derive(Component)]
struct Health;

fn setup_scoreboard(mut commands: Commands) {
    // Scoreboard: present counters
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Presents: ",
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
            top: SCORE_BASIC_TEXT_PADDING,
            left: SCORE_BASIC_TEXT_PADDING,
            ..default()
        }),
        CounterNice,
    ));
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Health: ",
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
            top: SCORE_BASIC_TEXT_PADDING,
            left: SCORE_NAUGHTY_TEXT_PADDING_LEFT,
            ..default()
        }),
        Health,
    ));
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

// Check for game over you win
fn end_game_win(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<(&mut Status, &mut Inventory), With<CharacterMarker>>,
) {
    let Ok((status, inventory)) = player_query.get_single() else {
        return;
    };

    if inventory.number_of_presents() != 5 {
        return;
    }

    if !status.game_over {
        return;
    }

    commands
        .spawn((NodeBundle {
            style: Style {
                align_self: AlignSelf::Stretch,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            // Display splash bitmap
            let splash_image: Handle<Image> = asset_server.load("images/splash.png");
            parent
                .spawn(ImageBundle {
                    style: Style {
                        align_self: AlignSelf::End,
                        width: Val::Px(480.),
                        height: Val::Px(288.),
                        justify_content: JustifyContent::SpaceAround,
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    image: UiImage::new(splash_image),
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(
                        TextBundle::from_section(
                            "You win!!",
                            TextStyle {
                                font_size: 35.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(10.0)),
                            align_self: AlignSelf::End,
                            ..default()
                        }),
                    );
                });
        });
}

// Check for game over you lose
fn end_game_lose(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<(&mut Status, &mut Inventory), With<CharacterMarker>>,
) {
    let Ok((status, inventory)) = player_query.get_single() else {
        return;
    };

    if inventory.number_of_presents() == 5 {
        return;
    }

    if !status.game_over {
        return;
    }

    commands
        .spawn((NodeBundle {
            style: Style {
                align_self: AlignSelf::Stretch,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            // Display splash bitmap
            let splash_image: Handle<Image> = asset_server.load("images/splash.png");
            parent
                .spawn(ImageBundle {
                    style: Style {
                        align_self: AlignSelf::End,
                        width: Val::Px(480.),
                        height: Val::Px(288.),
                        justify_content: JustifyContent::SpaceAround,
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    image: UiImage::new(splash_image),
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(
                        TextBundle::from_section(
                            "Oh no! You lost!",
                            TextStyle {
                                font_size: 35.0,
                                color: RED_TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(10.0)),
                            align_self: AlignSelf::End,
                            ..default()
                        }),
                    );
                });
        });
}
