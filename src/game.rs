use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
            .add_systems(Startup, (setup).chain());
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
