use avian2d::prelude::*;
use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_systems(Startup, setup_world)
        .add_systems(FixedUpdate, walk_down)
        .run()
}

fn setup_world(mut commands: Commands) {
    commands.spawn_scene(player());
    commands.spawn_scene_list(world());

    for y in 0..10 {
        for x in 0..10 {
            commands.spawn_scene(enemy(
                20.0,
                Vec2::new((x - 5) as f32 * 30.0, y as f32 * 30.0 + 500.0),
            ));
        }
    }
}

fn player() -> impl Scene {
    bsn! {
        Camera2d
    }
}

#[derive(Component, FromTemplate)]
struct EnemyMarker;

fn enemy(radius: f32, pos: Vec2) -> impl Scene {
    let sphere = Circle::new(radius);
    bsn! {
        Mesh2d(asset_value(sphere))
        MeshMaterial2d<ColorMaterial>(asset_value(Color::Srgba(Srgba { red: 1.0, green: 0.2, blue: 0.3, alpha: 1.0 })))
        Collider::from(sphere)
        Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0))
        EnemyMarker
        RigidBody::Dynamic
        // CollisionEventsEnabled

    }
}

fn walk_down(mut enemies: Query<Forces, With<EnemyMarker>>) {
    for mut enemy in &mut enemies {
        enemy.apply_force(Vec2::Y * -1000.0);
    }
}

fn world() -> impl SceneList {
    let outside_wall = Rectangle::new(500.0, 100000.0);

    bsn_list! {
        @wall(outside_wall, Vec2::X * -800.0)
        --
        @wall(outside_wall, Vec2::X * 800.0)
        --
        @death_zone(Rectangle::new(2000.0, 100.0), Vec2::Y * -400.0)
    }
}

fn wall(rect: Rectangle, position: Vec2) -> impl Scene {
    bsn! {
        Mesh2d(asset_value(rect))
        MeshMaterial2d<ColorMaterial>(asset_value(Color::Srgba(Srgba { red: 1.0, green: 0.2, blue: 0.3, alpha: 1.0 })))
        Collider::from(rect)
        Transform::from_translation(Vec3::new(position.x, position.y, 0.0))
        RigidBody::Static
    }
}

fn death_zone(rect: Rectangle, position: Vec2) -> impl Scene {
    bsn! {
        Mesh2d(asset_value(rect))
        MeshMaterial2d<ColorMaterial>(asset_value(Color::Srgba(Srgba { red: 1.0, green: 0.2, blue: 0.7, alpha: 0.2 })))
        Sensor
        Collider::from(rect)
        Transform::from_translation(Vec3::new(position.x, position.y, 0.0))
        CollisionEventsEnabled
        on(enter_deathzone)
    }
}

fn enter_deathzone(
    event: On<CollisionStart>,
    mut commands: Commands,
    enemies: Query<&EnemyMarker>,
) {
    if enemies.contains(event.collider2) {
        commands.entity(event.collider2).despawn();
    }
}
