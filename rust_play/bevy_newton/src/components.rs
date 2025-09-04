use bevy::prelude;

use crate::*;

#[derive(Component, Debug, Default)]
pub struct Pos(pub Vec2);

#[derive(Component, Debug, Default)]
pub struct PrevPos(pub Vec2);

#[derive(Component, Debug)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Self(1.) // Default to 1 kg
    }
}

#[derive(Component, Default)]
pub struct Particle {
    pub pos: Pos,
    pub prev_pos: PrevPos,
    pub mass: Mass,
}

impl Particle {
    pub fn new_with_pos_and_vel(pos: Vec2, vel: Vec2) -> Self {
        Self {
            pos: Pos(pos),
            prev_pos: PrevPos(pos - vel * DELTA_TIME),
            ..Default::default()
        }
    }
}
