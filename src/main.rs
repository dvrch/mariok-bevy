mod player;
mod track;
mod ui;
mod sounds;
mod logic;
mod items;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use player::PlayerPlugin;
use track::TrackPlugin;
use ui::UiPlugin;
use sounds::SoundsPlugin;
use logic::LogicPlugin;
use items::ItemsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Mario Kart Bevy 0.15".into(),
                ..default()
            }),
            ..default()
        }).set(bevy::render::RenderPlugin {
            render_creation: bevy::render::settings::RenderCreation::Automatic(bevy::render::settings::WgpuSettings {
                #[cfg(not(target_arch = "wasm32"))]
                features: bevy::render::settings::WgpuFeatures::empty(), // Keep it simple
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(TrackPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(UiPlugin)
        // .add_plugins(SoundsPlugin)
        .add_plugins(LogicPlugin)
        .add_plugins(ItemsPlugin)
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(mut commands: Commands) {
    // Sky
    commands.insert_resource(ClearColor(Color::srgb(0.53, 0.81, 0.92))); // Sky Blue

    // Global Light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 12_000.0,
            ..default()
        },
        Transform::from_xyz(50.0, 150.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 500.0,
    });
}
