use bevy::{prelude::*, window::WindowMode};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::Fullscreen(
                    // this won't work, panics
                    // MonitorSelection::Primary,
                    // this won't work either, panics
                    // MonitorSelection::Current,
                    // this works, though fullscreen is ignored on wayland
                    MonitorSelection::Index(0),
                    VideoModeSelection::Current,
                ),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .run();
}
