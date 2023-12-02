use bevy::prelude::*;

fn main() {
    // Minimal app with single window and non-default title
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Naughty And Nice".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .run();
}
