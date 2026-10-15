use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_world)
        .run()
}

fn spawn_world(mut commands: Commands) {
    commands.spawn_scene(world_scene());
    commands.spawn_scene(player_scene());
}

fn world_scene() -> impl Scene {
    bsn! {
        Mesh3d(asset_value(Cuboid::new(100.0, 0.1, 100.0)))
        MeshMaterial3d<StandardMaterial>(asset_value(Color::Srgba(Srgba::new(0.2, 0.8, 0.3, 1.0))))
    }
}

fn player_scene() -> impl Scene {
    bsn! {
        Mesh3d(asset_value(Sphere::new(1.0)))
        MeshMaterial3d<StandardMaterial>(asset_value(Color::Srgba(Srgba::new(0.2, 0.1, 0.8, 1.0))))
        Camera3d
        template_value(Transform::from_translation(Vec3::Y * 3.0).looking_at(Vec3::Z * -3.0 + Vec3::Y * 3.0, Vec3::Y))
    }
}
