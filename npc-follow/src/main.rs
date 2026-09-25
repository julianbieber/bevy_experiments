use avian2d::prelude::*;
use bevy::{color::palettes::css::GREEN_YELLOW, prelude::*};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin)
        .insert_resource(Gravity::ZERO)
        .add_systems(Startup, startup)
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
}

fn enemy_scene(transform: Transform) -> impl Scene {
    let t = Triangle2d::default();
    bsn! {
        Mesh2d(asset_value(t))
        MeshMaterial2d<ColorMaterial>(asset_value(Color::from(GREEN_YELLOW)))
        template_value(transform)
        template_value(RigidBody::Dynamic)
        Collider::triangle(t.vertices[0], t.vertices[1], t.vertices[2])
    }
}
