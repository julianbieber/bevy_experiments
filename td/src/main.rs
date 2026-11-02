use avian2d::prelude::*;
use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .run()
}

fn setup_world(mut commands: Commands) {}

fn world() -> impl Scene {
    bsn! {}
}
