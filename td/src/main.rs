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
    let outside_wall = Rectangle::new(200.0, 100000.0);

    bsn! {
        Mesh2d(asset_value(outside_wall))
        Collider::rectangle(outside_wall.half_size.x * 2.0, outside_wall.half_size.y * 2.0)
        Transform::from_translation(Vec3::X * -200.0)
        --
    }
}
