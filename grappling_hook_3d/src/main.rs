use avian3d::prelude::*;
use bevy::{image::ImageLoaderSettings, input::mouse::AccumulatedMouseMotion, prelude::*};

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
    commands.spawn_scene(bsn! {
    template_value(Transform::from_scale(Vec3::splat(1.0)))
    Children[
        {tunnel_z()}
    ]
    });
    commands.spawn_scene(player_scene());
    commands.spawn(Camera3d::default());
}

pub fn material_wall() -> impl Scene {
    bsn! {
        template(|ctx| {
            let linear = |settings: &mut ImageLoaderSettings| settings.is_srgb = false;
            let assets = ctx.resource::<AssetServer>();
            let base_color: Handle<Image> = assets.load("output/wall/base_color.png");
            let normal: Handle<Image> = assets.load_builder().with_settings(linear).load("output/wall/normal.png");
            let orm: Handle<Image> = assets.load_builder().with_settings(linear).load("output/wall/orm.png");
            let emissive: Handle<Image> = assets.load("output/wall/emissive.png");
            let transmission: Handle<Image> = assets.load_builder().with_settings(linear).load("output/wall/transmission.png");
            let specular: Handle<Image> = assets.load("output/wall/specular.png");
            let clearcoat: Handle<Image> = assets.load_builder().with_settings(linear).load("output/wall/clearcoat.png");
            let clearcoat_normal: Handle<Image> = assets.load_builder().with_settings(linear).load("output/wall/clearcoat_normal.png");
            let anisotropy: Handle<Image> = assets.load_builder().with_settings(linear).load("output/wall/anisotropy.png");
            let depth: Handle<Image> = assets.load_builder().with_settings(linear).load("output/wall/depth.png");
            let material = StandardMaterial {
                base_color_texture: Some(base_color.clone()),
                normal_map_texture: Some(normal.clone()),
                occlusion_texture: Some(orm.clone()),
                metallic_roughness_texture: Some(orm.clone()),
                emissive_texture: Some(emissive.clone()),
                specular_transmission_texture: Some(transmission.clone()),
                thickness_texture: Some(transmission.clone()),
                diffuse_transmission_texture: Some(transmission.clone()),
                specular_tint_texture: Some(specular.clone()),
                specular_texture: Some(specular.clone()),
                clearcoat_texture: Some(clearcoat.clone()),
                clearcoat_roughness_texture: Some(clearcoat.clone()),
                clearcoat_normal_texture: Some(clearcoat_normal.clone()),
                anisotropy_texture: Some(anisotropy.clone()),
                depth_map: Some(depth.clone()),
                emissive: LinearRgba::WHITE,
                perceptual_roughness: 1.0,
                metallic: 1.0,
                reflectance: 1.0,
                clearcoat: 1.0,
                clearcoat_perceptual_roughness: 1.0,
                anisotropy_strength: 1.0,
                specular_transmission: 1.0,
                diffuse_transmission: 1.0,
                thickness: 1.0,
                ..default()
            };
            let handle = ctx.resource_mut::<Assets<StandardMaterial>>().add(material);
            Ok(MeshMaterial3d(handle))
        })
    }
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
            material_wall()
            template_value(Transform::from_translation(Vec3::Y * 10.0))
        ),
        (
            #Bottom
            Mesh3d(asset_value(Cuboid::new(10.0, 0.1, 100.0)))
            template_value(wide)
            template_value(RigidBody::Static)
            material_wall()
            template_value(Transform::from_translation(Vec3::Y * 0.0))
        ),
        (
            #Left
            Mesh3d(asset_value(Cuboid::new(0.1, 10.0, 100.0)))
            template_value(high.clone())
            template_value(RigidBody::Static)
            material_wall()
            template_value(Transform::from_translation(Vec3::Y * 5.0 + Vec3::X * -5.0))
        ),
        (
            #Right
            Mesh3d(asset_value(Cuboid::new(0.1, 10.0, 100.0)))
            template_value(high)
            template_value(RigidBody::Static)
            material_wall()
            template_value(Transform::from_translation(Vec3::Y * 5.0 + Vec3::X * 5.0))
        ),
        (
            #Light
            template_value(PointLight{
                color: Color::Oklaba(Oklaba { lightness: 0.8, a: 0.8, b: 0.7, alpha: 1.0 }),
                range: 10.0,
                intensity: 10.0,
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
    let mut forward = camera.forward().with_y(0.0);
    if forward.length_squared() == 0.0 {
        return;
    }
    forward = forward.normalize();
    let mut right = camera.right().with_y(0.0);
    if right.length_squared() == 0.0 {
        return;
    }
    right = right.normalize();

    if keys.pressed(KeyCode::KeyW) {
        player.apply_linear_impulse(forward);
    }
    if keys.pressed(KeyCode::KeyS) {
        player.apply_linear_impulse(-forward);
    }
    if keys.pressed(KeyCode::KeyA) {
        player.apply_linear_impulse(-right);
    }
    if keys.pressed(KeyCode::KeyD) {
        player.apply_linear_impulse(right);
    }
}
