use avian3d::prelude::*;
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_systems(Startup, spawn_world)
        .add_systems(Update, (sync_camera, camera_rotation))
        .add_systems(FixedUpdate, player_movement)
        .run()
}

fn spawn_world(mut commands: Commands) {
    commands.spawn_scene_list(tunnel_z());
    commands.spawn_scene(player_scene());
    commands.spawn(Camera3d::default());
}

fn tunnel_z() -> impl SceneList {
    let wide = Collider::cuboid(10.0, 0.1, 100.0);
    let high = Collider::cuboid(0.1, 10.0, 100.0);
    bsn_list! {
        (
            #Top
            Mesh3d(asset_value(Cuboid::new(10.0, 0.1, 100.0)))
            template_value(wide.clone())
            template_value(RigidBody::Static)
            MeshMaterial3d<StandardMaterial>(asset_value(Color::Srgba(Srgba::new(0.2, 0.8, 0.3, 1.0))))
            template_value(Transform::from_translation(Vec3::Y * 10.0))
        ),
        (
            #Bottom
            Mesh3d(asset_value(Cuboid::new(10.0, 0.1, 100.0)))
            template_value(wide)
            template_value(RigidBody::Static)
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
            template_value(high.clone())
            template_value(RigidBody::Static)
            MeshMaterial3d<StandardMaterial>(asset_value(Color::Srgba(Srgba::new(0.2, 0.8, 0.3, 1.0))))
            template_value(Transform::from_translation(Vec3::Y * 5.0 + Vec3::X * -5.0))
        ),
        (
            #Right
            Mesh3d(asset_value(Cuboid::new(0.1, 10.0, 100.0)))
            template_value(high)
            template_value(RigidBody::Static)
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

#[derive(Component, FromTemplate)]
struct PlayerMarker;

#[derive(Component, FromTemplate)]
struct FollowCamera {
    rotation: f32,
    look_height: f32,
}

fn player_scene() -> impl Scene {
    bsn! {
        PlayerMarker
        Mesh3d(asset_value(Sphere::new(1.0)))
        template_value(RigidBody::Dynamic)
        template_value(Collider::sphere(1.0))
        MeshMaterial3d<StandardMaterial>(asset_value(Color::Srgba(Srgba::new(0.2, 0.1, 0.8, 1.0))))
        template_value(Transform::from_translation(Vec3::Y * 3.0))
        FollowCamera
    }
}

fn sync_camera(
    target: Single<(&Transform, &FollowCamera), Without<Camera>>,
    mut camera: Single<&mut Transform, (With<Camera>, Without<FollowCamera>)>,
) {
    let offset = Vec3::Y * 2.5 + Vec3::Z * 2.5;
    camera.translation = target.0.translation + offset.rotate_y(-target.1.rotation);
    // camera.translation = target.0.translation + offset;
    camera.look_at(
        target.0.translation + Vec3::Y * (1.4 + target.1.look_height),
        Vec3::Y,
    );
}

fn camera_rotation(
    mouse: Res<AccumulatedMouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut follow: Single<&mut FollowCamera>,
) {
    if mouse_buttons.pressed(MouseButton::Right) {
        follow.rotation += mouse.delta.x * 0.01;
        follow.look_height -= mouse.delta.y * 0.01;
    }
}

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    player: Single<Forces, With<PlayerMarker>>,
    camera: Single<&Transform, With<Camera>>,
) {
    let mut player = player.into_inner();
    let camera = camera.into_inner();

    if keys.pressed(KeyCode::KeyW) {
        player.apply_linear_impulse(camera.forward().as_vec3());
    }
    if keys.pressed(KeyCode::KeyS) {
        player.apply_linear_impulse(-camera.forward().as_vec3());
    }
    if keys.pressed(KeyCode::KeyA) {
        player.apply_linear_impulse(-camera.right().as_vec3());
    }
    if keys.pressed(KeyCode::KeyD) {
        player.apply_linear_impulse(camera.right().as_vec3());
    }
}
