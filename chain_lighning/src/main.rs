use bevy::{
    color::palettes::tailwind::{BLUE_200, BLUE_300, RED_200},
    prelude::*,
    window::PrimaryWindow,
};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_message::<MapClick>()
        .add_systems(Startup, (spawn_player, spawn_enemies))
        .add_systems(
            FixedUpdate,
            (
                map_clicks,
                eval_map_clicks,
                eval_chain,
                position_chains,
                move_player,
                tick_chains,
            ),
        )
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

fn move_player(
    mut player: Single<&mut Transform, With<PlayerMarker>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let mut o = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        o.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        o.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        o.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        o.x += 1.0;
    }
    player.translation += Vec3::new(o.x, o.y, 0.0);
}

fn spawn_enemies(mut commands: Commands) {
    for x in [20.0, 30.0, 45.0, -20.0, -60.0, 80.0, 70.0] {
        for y in [20.0, 30.0, 45.0, -20.0, -60.0, -80.0, 55.0] {
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

#[derive(Message)]
struct MapClick(Vec2);

fn map_clicks(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut map_clicks: MessageWriter<MapClick>,
    player: Single<&Transform, With<Camera2d>>,
) {
    if mouse.pressed(MouseButton::Left) {
        let cursor = window.cursor_position().unwrap();
        let mut location = cursor - Vec2::new(window.width() * 0.5, window.height() * 0.5);
        location.y *= -1.0;
        location += player.translation.xy();
        map_clicks.write(MapClick(location));
    }
}

fn eval_map_clicks(
    mut commands: Commands,
    mut map_clicks: MessageReader<MapClick>,
    player: Single<(&Transform, Entity), (Without<EnemyMarker>, With<PlayerMarker>)>,
    enemies: Query<(&Transform, Entity), (With<EnemyMarker>, Without<PlayerMarker>)>,
) {
    for click in map_clicks.read() {
        for (enemy, enemy_e) in &enemies {
            let enemy = enemy.translation.xy();
            if enemy.distance_squared(click.0) < 300.0 {
                let chain = ChainLightning {
                    remaining: 2,
                    fork: 3,
                    chained: false,
                    origin: player.1,
                    destination: enemy_e,
                    visited: vec![enemy_e],
                    time_left: Timer::from_seconds(2.0, TimerMode::Once),
                };
                commands.spawn_scene(chain_lightning_scene(
                    player.0.translation.xy(),
                    enemy,
                    chain,
                ));
                break;
            }
        }
    }
}

fn tick_chains(
    mut chains: Query<(&mut ChainLightning, Entity)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut chain, e) in &mut chains {
        chain.time_left.tick(time.delta());
        if chain.time_left.is_finished() {
            commands.entity(e).despawn();
        }
    }
}

fn eval_chain(
    mut commands: Commands,
    enemies: Query<(&Transform, Entity), (With<EnemyMarker>, Without<PlayerMarker>)>,
    mut chains: Query<&mut ChainLightning>,
) {
    for mut chain in &mut chains {
        if chain.remaining == 0 || chain.chained {
            continue;
        }

        let mut forks_remaining = chain.fork;
        let current = enemies.get(chain.destination).unwrap();
        for (enemy, enemy_e) in &enemies {
            let enemy = enemy.translation.xy();
            if enemy.distance_squared(current.0.translation.xy()) < 3000.0
                && chain.visited.iter().find(|v| **v == enemy_e).is_none()
            {
                let mut new_visited = chain.visited.clone();
                new_visited.push(enemy_e);
                let chain = ChainLightning {
                    remaining: chain.remaining - 1,
                    fork: 2,
                    chained: false,
                    origin: current.1,
                    destination: enemy_e,
                    visited: new_visited,
                    time_left: chain.time_left.clone(),
                };
                commands.spawn_scene(chain_lightning_scene(
                    current.0.translation.xy(),
                    enemy,
                    chain,
                ));
                forks_remaining -= 1;
                if forks_remaining == 0 {
                    break;
                }
            }
        }
        chain.chained = true;
    }
}

#[derive(Component, Clone)]
struct ChainLightning {
    remaining: u8,
    fork: u8,
    chained: bool,
    origin: Entity,
    destination: Entity,
    visited: Vec<Entity>,
    time_left: Timer,
}

/// Do not use the default chain lightning, it only exists to be replaced by the actual value in bsn
impl Default for ChainLightning {
    fn default() -> Self {
        Self {
            remaining: Default::default(),
            fork: Default::default(),
            chained: Default::default(),
            origin: Entity::PLACEHOLDER,
            destination: Entity::PLACEHOLDER,
            visited: Default::default(),
            time_left: Timer::from_seconds(0.0, TimerMode::Once),
        }
    }
}

fn position_chains(
    mut chains: Query<(&mut Transform, &ChainLightning)>,
    positioned: Query<
        &Transform,
        (
            Or<(With<PlayerMarker>, With<EnemyMarker>)>,
            Without<ChainLightning>,
        ),
    >,
) {
    for (mut chain_transfrom, chain) in &mut chains {
        let start = positioned.get(chain.origin).unwrap().translation.xy();
        let end = positioned.get(chain.destination).unwrap().translation.xy();

        let center = start + (end - start) * 0.5;
        let length = (end - start).length();
        let rotation = Quat::from_rotation_z((end - start).to_angle());
        let transform = Transform::from_translation(Vec3::new(center.x, center.y, 0.0))
            .with_rotation(rotation)
            .with_scale(Vec3::X * length + Vec3::Y * 3.0);
        *chain_transfrom = transform;
    }
}

fn chain_lightning_scene(start: Vec2, end: Vec2, chain: ChainLightning) -> impl Scene {
    let center = start + (end - start) * 0.5;
    let length = (end - start).length();
    let rotation = Quat::from_rotation_z((end - start).to_angle());
    let transform = Transform::from_translation(Vec3::new(center.x, center.y, 0.0))
        .with_rotation(rotation)
        .with_scale(Vec3::X * length + Vec3::Y * 3.0);
    bsn! {
        template_value(transform)
        template_value(chain)
        Mesh2d(asset_value(Rectangle::from_length(1.0)))
        MeshMaterial2d<ColorMaterial>(asset_value(Color::from(BLUE_200)))
    }
}
