use bevy::{core::FixedTimestep, input::system::exit_on_esc_system, prelude::*};

const TIMESTEP: f32 = 1.0 / 60.0;

#[derive(Component)]
struct Enemy {
    speed: f32,
}

fn startup(mut commands: Commands, windows: Res<Windows>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let win = windows.get_primary().expect("no primary window");
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-win.width(), 0.0, 0.0),
                scale: Vec3::splat(30.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy { speed: 200.0 });
}

fn entity_movement_system(mut enemy_query: Query<(&Enemy, &mut Transform)>) {
    for (enemy, mut transform) in enemy_query.iter_mut() {
        transform.translation.x += enemy.speed * TIMESTEP;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 1.0)))
        .add_startup_system(startup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIMESTEP as f64))
                .with_system(entity_movement_system),
        )
        .add_system(exit_on_esc_system)
        .run();
}
