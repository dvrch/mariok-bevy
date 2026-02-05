use bevy::prelude::*;
use crate::player::Kart;

pub struct SoundsPlugin;

impl Plugin for SoundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_sounds)
           .add_systems(Update, update_sounds);
    }
}

#[derive(Component)]
struct EngineSound;

#[derive(Component)]
struct DriftSound;

fn setup_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Engine
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/engine.wav")),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            paused: true,
            ..default()
        },
        EngineSound,
    ));

    // Drift
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/drifting.wav")),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            paused: true,
            volume: bevy::audio::Volume::new(0.0),
            ..default()
        },
        DriftSound,
    ));
}

fn update_sounds(
    kart_query: Query<&Kart>,
    mut engine_query: Query<(&mut PlaybackSettings, &AudioSink), (With<EngineSound>, Without<DriftSound>)>,
    mut drift_query: Query<(&mut PlaybackSettings, &AudioSink), (With<DriftSound>, Without<EngineSound>)>,
) {
    if let Ok(kart) = kart_query.get_single() {
        // Engine
        if let Ok((mut settings, sink)) = engine_query.get_single_mut() {
            if kart.speed.abs() > 0.1 {
                settings.paused = false;
                let pitch = 0.7 + (kart.speed.abs() / 40.0);
                sink.set_speed(pitch.min(2.0));
                sink.set_volume(0.3 + (kart.speed.abs() / 80.0));
            } else {
                settings.paused = true;
            }
        }

        // Drift
        if let Ok((mut settings, sink)) = drift_query.get_single_mut() {
            if kart.drift_dir != 0.0 {
                settings.paused = false;
                sink.set_volume(0.6);
            } else {
                settings.paused = true;
                sink.set_volume(0.0);
            }
        }
    }
}
