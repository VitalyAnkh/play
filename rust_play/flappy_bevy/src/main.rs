use bevy::prelude::*;

mod components;
mod constants;
mod plugin;
mod setup;
mod system;
mod utils;

use crate::plugin::*;
use crate::setup::*;
use crate::system::*;

fn main() {
    App::new()
        .add_plugins(MyPlugin)
        .add_systems(Startup, (hello_world, setup))
        .add_systems(Update, blink_space_bar_text)
        .run();
}

fn hello_world() {
    println!("hello world from vitalyr");
}
