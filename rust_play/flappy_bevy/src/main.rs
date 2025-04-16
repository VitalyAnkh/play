use bevy::prelude::*;

mod components;
mod constants;
mod plugin;
mod setup;

use crate::plugin::*;
use crate::setup::*;

fn main() {
    App::new()
        .add_plugins(MyPlugin)
        .add_systems(Startup, (hello_world, setup))
        .run();
}

fn hello_world() {
    println!("hello world from vitalyr");
}
