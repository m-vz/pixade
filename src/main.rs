use bevy::prelude::*;

use crate::game::GamePlugin;

mod game;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
