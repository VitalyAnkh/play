use bevy::{math::VectorSpace, prelude::*};
use bevy_newton::*;

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

    let white = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        ..default()
    });

    commands
        .spawn((Mesh3d(sphere.clone()), MeshMaterial3d(white.clone())))
        .insert(Particle::new_with_pos_and_vel(
            Vec2::ZERO,
            Vec2::new(2., 0.),
        ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0., 0., 100.)),
        Projection::Orthographic(OrthographicProjection {
            scale: 0.01,
            ..OrthographicProjection::default_3d()
        }),
    ));
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        // .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_plugins(NewtonPlugin)
        .run();
}
