use bevy::prelude::*;
use bevy::winit::WinitSettings;

fn main() {
    App::new()
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins.build())
        .run();
}
