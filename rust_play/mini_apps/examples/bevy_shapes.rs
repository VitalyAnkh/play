use bevy::prelude::*;

#[derive(Component)]
struct MyComponent(u32);

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(MyComponent(1));
}
