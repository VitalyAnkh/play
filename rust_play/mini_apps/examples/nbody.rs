use bevy::prelude::Circle;
use bevy::prelude::*;
use rand::Rng;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const NUM_BODIES: usize = 50;
const G: f32 = 6.67430e-11; // Gravitational constant
const MASS_RANGE: std::ops::Range<f32> = 1e10..1e11;
const SOFTENING: f32 = 5.0; // Softening factor to avoid division by zero

#[derive(Component)]
struct Body {
    velocity: Vec2,
    mass: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "N-body Simulation".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (update_bodies, update_positions))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let mut rng = rand::thread_rng();

    for _ in 0..NUM_BODIES {
        let mass = rng.gen_range(MASS_RANGE);
        let size = (mass.log10() - 9.0) * 2.0; // Adjust size based on mass

        commands.spawn((
            Mesh2d(meshes.add(Mesh::from(Circle::new(size))).into()),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
            Transform::from_xyz(
                rng.gen_range(-WINDOW_WIDTH / 2.0..WINDOW_WIDTH / 2.0),
                rng.gen_range(-WINDOW_HEIGHT / 2.0..WINDOW_HEIGHT / 2.0),
                0.0,
            ),
            Body {
                velocity: Vec2::new(rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0)),
                mass,
            },
        ));
    }
}

fn update_bodies(mut query: Query<(&mut Body, &Transform)>) {
    // This doesn't interfere the original query's usage if `reborrow` is not active
    // let reborrow = query.reborrow();
    let bodies: Vec<(Vec2, f32)> = query
        .iter()
        .map(|(body, transform)| (transform.translation.truncate(), body.mass))
        .collect();

    for (mut body, transform) in query.iter_mut() {
        let pos = transform.translation.truncate();
        let mut acceleration = Vec2::ZERO;

        for (other_pos, other_mass) in &bodies {
            if *other_pos != pos {
                let r = *other_pos - pos;
                let r_squared = r.length_squared().max(SOFTENING * SOFTENING);
                acceleration += G * other_mass / r_squared * r.normalize();
            }
        }

        body.velocity += acceleration;
    }
}

fn update_positions(mut query: Query<(&Body, &mut Transform)>, time: Res<Time>) {
    for (body, mut transform) in query.iter_mut() {
        transform.translation += body.velocity.extend(0.0) * time.delta_secs();

        // Wrap around screen edges
        let mut pos = transform.translation;
        pos.x = (pos.x + WINDOW_WIDTH / 2.0) % WINDOW_WIDTH - WINDOW_WIDTH / 2.0;
        pos.y = (pos.y + WINDOW_HEIGHT / 2.0) % WINDOW_HEIGHT - WINDOW_HEIGHT / 2.0;
        transform.translation = pos;
    }
}
