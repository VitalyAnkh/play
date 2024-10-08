use bevy::prelude::*;
use bevy_physics::*;

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut sphere_mesh = Sphere::new(1.0).mesh().build();
    sphere_mesh
        .generate_tangents()
        .expect("Failed to generate tangents");
    let sphere = meshes.add(sphere_mesh);
    // let sphere = meshes.add(Mesh::from(shape::Icosphere {
    //     radius: 0.5,
    //     subdivisions: 4,
    // }));

    let white = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        ..Default::default()
    });

    commands
        .spawn(PbrBundle {
            mesh: sphere.clone(),
            material: white.clone(),
            ..Default::default()
        })
        .insert(Pos(Vec2::ZERO));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 100.)),
        projection: Projection::Orthographic(OrthographicProjection {
            scale: 0.01,
            ..Default::default()
        }),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .run();
}
