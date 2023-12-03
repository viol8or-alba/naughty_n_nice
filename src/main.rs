use bevy::prelude::*;
use setup::InitialSetup;

mod animation;
mod characters;
mod control_input;
mod markers;
mod setup;

fn main() {
    // Minimal app with single window and non-default title
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Naughty And Nice".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(
                    // This sets image filtering to nearest
                    // This is done to prevent textures with low resolution (e.g. pixel art) from being blurred
                    // by linear filtering.
                    ImagePlugin::default_nearest(),
                ),
            InitialSetup,
        ))
        .run();
}
