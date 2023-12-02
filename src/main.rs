use bevy::prelude::*;
use setup::InitialSetup;

mod setup;
mod markers;
mod animation;
mod characters;

fn main() {
    // Minimal app with single window and non-default title
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Naughty And Nice".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }), InitialSetup))
        .run();
}
