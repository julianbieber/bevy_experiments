use bevy::{
    color::palettes::tailwind::{BLUE_300, RED_200},
    prelude::*,
};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_player, spawn_enemies))
        .run()
}

fn spawn_player(mut commands: Commands) {
    commands.spawn_scene(player_scene());
}

#[derive(Component, FromTemplate)]
struct PlayerMarker;

fn player_scene() -> impl Scene {
    bsn! {
        Mesh2d(asset_value(Circle::new(10.0)))
        MeshMaterial2d<ColorMaterial>(asset_value(Color::from(BLUE_300)))
        Camera2d
        PlayerMarker
    }
}

fn spawn_enemies(mut commands: Commands) {
    for x in [20.0, 30.0, 45.0, -20.0, -60.0] {
        for y in [20.0, 30.0, 45.0, -20.0, -60.0] {
            commands.spawn_scene(enemy_scene(Vec2::new(x, y) * 3.0));
        }
    }
}

#[derive(Component, FromTemplate)]
struct EnemyMarker;

fn enemy_scene(pos: Vec2) -> impl Scene {
    let transform = Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0));
    bsn! {
        Mesh2d(asset_value(Circle::new(5.0)))
        MeshMaterial2d<ColorMaterial>(asset_value(Color::from(RED_200)))
        template_value(transform)
        EnemyMarker
    }
}
