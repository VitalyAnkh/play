use bevy::prelude::*;

mod components;

pub use components::*;

pub const DELTA_TIME: f32 = 1. / 60.;

// Move objects in the physics world
fn simulate(mut query: Query<(&mut Pos, &mut PrevPos, &Mass)>) {
    for (mut pos, mut prev_pos, mass) in query.iter_mut() {
        let gravity = Vec2::new(0., -9.81);
        let gravitation_force = mass.0 * gravity;
        let external_forces = gravitation_force;
        let velocity = (pos.0 - prev_pos.0) / DELTA_TIME + DELTA_TIME * external_forces / mass.0;
        prev_pos.0 = pos.0;
        pos.0 = prev_pos.0 + velocity * DELTA_TIME;
    }
    println!("debug!");
}

// Copy positions from the physics world to render world
fn sync_transform(mut query: Query<(&mut Transform, &Pos)>) {
    for (mut transform, pos) in query.iter_mut() {
        transform.translation = pos.0.extend(0.);
    }
    // println!("debug!!!!!!!!!!!!!!!");
}

#[derive(Debug, Default)]
pub struct NewtonPlugin;

impl Plugin for NewtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (simulate, sync_transform))
            // configure our fixed timestep schedule to run every DELTA_TIME
            .insert_resource(Time::<Fixed>::from_seconds(DELTA_TIME as f64));
    }
}
