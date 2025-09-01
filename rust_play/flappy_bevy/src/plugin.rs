use bevy::prelude::*;

use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Flappy Bevy".to_string(),
                resolution: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).into(),
                // resizable: false,
                ..default()
            }),
            ..default()
        }));
    }
}
