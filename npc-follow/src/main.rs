use avian2d::prelude::*;
use bevy::{color::palettes::css::GREEN_YELLOW, prelude::*};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin)
        .add_systems(Startup, startup)
        .run()
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    let t = Triangle2d::default();

    commands.spawn((
        Mesh2d(meshes.add(t)),
        MeshMaterial2d(materials.add(Color::from(GREEN_YELLOW))),
        Transform::from_scale(Vec3::ONE * 100.0),
        RigidBody::Dynamic,
        Collider::triangle(t.vertices[0], t.vertices[1], t.vertices[2]),
    ));
}
