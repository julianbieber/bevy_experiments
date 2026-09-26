use avian2d::prelude::*;
use bevy::{
    color::palettes::css::{BLUE_VIOLET, GREEN_YELLOW},
    prelude::*,
};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        // .add_plugins(PhysicsDebugPlugin)
        .insert_resource(Gravity::ZERO)
        .add_systems(Startup, startup)
        .add_systems(Update, follow_camera)
        .add_systems(FixedUpdate, (move_player, move_enemies))
        .run()
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);

    for y in 0..10 {
        for x in 0..10 {
            commands.spawn_scene(enemy_scene(
                Transform::from_scale(Vec3::ONE * 100.0).with_translation(
                    Vec3::new(x as f32, y as f32, 0.0) * 100.0 - Vec3::new(500.0, 500.0, 0.0),
                ),
            ));
        }
    }

    commands.spawn_scene(player_scene(Transform::from_translation(Vec3::new(
        0.0, 1000.0, 0.0,
    ))));
}

#[derive(Component, FromTemplate)]
struct EnemyMarker;

#[derive(Component, FromTemplate)]
struct PlayerMarker;

fn enemy_scene(transform: Transform) -> impl Scene {
    let t = Triangle2d::default();
    bsn! {
        Mesh2d(asset_value(t))
        MeshMaterial2d<ColorMaterial>(asset_value(Color::from(GREEN_YELLOW)))
        template_value(transform)
        template_value(RigidBody::Dynamic)
        Collider::triangle(t.vertices[0], t.vertices[1], t.vertices[2])
        EnemyMarker
    }
}

fn player_scene(transform: Transform) -> impl Scene {
    let c = Circle::new(20.0);
    let collider = Collider::circle(c.radius);
    bsn! {
        Mesh2d(asset_value(c))
        MeshMaterial2d<ColorMaterial>(asset_value(Color::from(BLUE_VIOLET)))
        template_value(transform)
        template_value(RigidBody::Dynamic)
        template_value(collider)
        Mass(100000.0)
        PlayerMarker
    }
}

fn follow_camera(
    player: Single<&Transform, (With<PlayerMarker>, Without<Camera>)>,
    mut camera: Single<&mut Transform, (With<Camera>, Without<PlayerMarker>)>,
) {
    camera.translation = player.into_inner().translation;
    camera.scale = Vec3::ONE * 10.0;
}

fn move_player(player: Single<Forces, With<PlayerMarker>>, input: Res<ButtonInput<KeyCode>>) {
    let mut player = player.into_inner();
    let mut v = Vec2::new(0.0, 0.0);
    if input.pressed(KeyCode::KeyW) {
        v.y += 10.0;
    }
    if input.pressed(KeyCode::KeyS) {
        v.y -= 10.0;
    }
    if input.pressed(KeyCode::KeyA) {
        v.x -= 10.0;
    }
    if input.pressed(KeyCode::KeyD) {
        v.x += 10.0;
    }
    v *= 10260.0;
    player.apply_linear_impulse(v);
}

fn move_enemies(
    mut enemies: Query<(Forces, &Transform), With<EnemyMarker>>,
    player: Single<&Transform, (With<PlayerMarker>, Without<EnemyMarker>)>,
) {
    let player = player.translation.xy();

    for (mut e_forces, e_transform) in &mut enemies {
        let e = e_transform.translation.xy();
        let diff = (player - e).normalize();

        e_forces.apply_linear_impulse(diff * 2024.0);
    }
}
