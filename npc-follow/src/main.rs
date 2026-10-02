use avian2d::prelude::*;
use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css::{BLUE_VIOLET, GREEN_YELLOW},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        // .add_plugins(PhysicsDebugPlugin)
        .insert_resource(Gravity::ZERO)
        .add_systems(Startup, (startup, spawn_backgrund_tilemap))
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
        let forward: Vec2 = (e_transform.rotation * Vec3::Y).xy();
        let to_player = (player - e_transform.translation.xy()).normalize();
        let forward_dot = forward.dot(to_player);
        let right: Vec2 = (e_transform.rotation * Vec3::X).xy();
        let sign = -right.dot(to_player).signum();
        let max_angle = forward_dot.clamp(-1.0, 1.0).acos();
        if e_forces.linear_velocity().length_squared() > 0.0 {
            *e_forces.linear_velocity_mut() = e_forces
                .linear_velocity()
                .rotate_towards(forward, sign * max_angle);
        }
        e_forces.apply_angular_impulse((sign * 60000.3));

        e_forces.apply_linear_impulse(forward * 2048.0);
    }
}

fn spawn_backgrund_tilemap(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut tilemap_data: Vec<Option<TileData>> = Vec::new();
    for y in 0..32 {
        for x in 0..32 {
            tilemap_data.push(Some(TileData::from_tileset_index((x * y + 3) % 4)))
        }
    }
    commands.spawn((
        TilemapChunk {
            chunk_size: uvec2(32, 32),
            tile_display_size: uvec2(128, 128) * 8,
            tileset: images.add(generate_background_image()),
            alpha_mode: bevy::sprite_render::AlphaMode2d::Opaque,
        },
        TilemapChunkTileData(tilemap_data),
    ));
}

fn generate_background_image() -> Image {
    let mut data = Vec::with_capacity(8 * 8 * 4 * 4);
    for y in 0..8 {
        for x in 0..8 * 4 {
            data.push((x + 80) * (y + 2));
            data.push(x * y * 32);
            data.push(x * y * 67);
            data.push(255);
        }
    }
    let mut image = Image::new(
        Extent3d {
            width: 8,
            height: 8 * 4,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::all(),
    );
    image.reinterpret_stacked_2d_as_array(4).unwrap();

    image
}
