use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_world)
        .run()
}

fn spawn_world(mut commands: Commands) {
    commands.spawn_scene_list(tunnel_z());
    commands.spawn_scene(player_scene());
}

fn tunnel_z() -> impl SceneList {
    bsn_list! {
        (
            #Top
            Mesh3d(asset_value(Cuboid::new(10.0, 0.1, 100.0)))
            MeshMaterial3d<StandardMaterial>(asset_value(Color::Srgba(Srgba::new(0.2, 0.8, 0.3, 1.0))))
            template_value(Transform::from_translation(Vec3::Y * 10.0))
        ),
        (
            #Bottom
            Mesh3d(asset_value(Cuboid::new(10.0, 0.1, 100.0)))
            MeshMaterial3d<StandardMaterial>(asset_value(StandardMaterial{
                base_color: Color::Oklaba(Oklaba { lightness: 0.4, a: -0.4, b: 0.4, alpha: 1.0 }),
                metallic: 0.5,
                reflectance: 1.0,
                ..Default::default()}))
            template_value(Transform::from_translation(Vec3::Y * 0.0))
        ),
        (
            #Left
            Mesh3d(asset_value(Cuboid::new(0.1, 10.0, 100.0)))
            MeshMaterial3d<StandardMaterial>(asset_value(Color::Srgba(Srgba::new(0.2, 0.8, 0.3, 1.0))))
            template_value(Transform::from_translation(Vec3::Y * 5.0 + Vec3::X * -5.0))
        ),
        (
            #Right
            Mesh3d(asset_value(Cuboid::new(0.1, 10.0, 100.0)))
            MeshMaterial3d<StandardMaterial>(asset_value(Color::Srgba(Srgba::new(0.2, 0.8, 0.3, 1.0))))
            template_value(Transform::from_translation(Vec3::Y * 5.0 + Vec3::X * 5.0))
        ),
        (
            #Light
            template_value(PointLight{
                color: Color::Oklaba(Oklaba { lightness: 0.8, a: 0.8, b: 0.7, alpha: 1.0 }),
                range: 10.0,
                ..Default::default()
            })
            template_value(Transform::from_translation(Vec3::Z * -20.0 + Vec3::Y * 4.0 ))
        )
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
