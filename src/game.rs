use bevy::prelude::*;
use bevy_persistent::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Serialize, Deserialize, Default)]
struct Settings {
    emulate_screen: bool,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
            .add_systems(Startup, (setup).chain());
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(
        Persistent::<Settings>::builder()
            .name("settings")
            .format(StorageFormat::Toml)
            .path("settings.toml")
            .default(Settings::default())
            .build()
            .expect("Could not read or create settings file"),
    )
}
