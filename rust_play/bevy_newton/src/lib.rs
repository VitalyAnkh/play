use bevy::prelude::*;

pub const DELTA_TIME: f32 = 1. / 60.;
#[derive(Component, Debug, Default)]
pub struct Pos(pub Vec2);

#[derive(Component, Debug, Default)]
pub struct PrevPos(pub Vec2);
