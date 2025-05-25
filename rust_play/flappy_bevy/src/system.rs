use bevy::prelude::*;

pub fn blink_space_bar_text(time: Res<Time>) {
    dbg!(time.delta());
}
