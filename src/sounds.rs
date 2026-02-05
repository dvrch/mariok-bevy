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

fn setup_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/engine.wav")),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            paused: true,
            ..default()
        },
        EngineSound,
    ));
}

fn update_sounds(
    kart_query: Query<&Kart>,
    mut audio_query: Query<(&mut PlaybackSettings, &AudioSink), With<EngineSound>>,
) {
    if let Ok(kart) = kart_query.get_single() {
        if let Ok((mut settings, sink)) = audio_query.get_single_mut() {
            if kart.current_speed.abs() > 0.1 {
                settings.paused = false;
                sink.set_speed(0.8 + (kart.current_speed.abs() / 50.0));
                sink.set_volume(0.5 + (kart.current_speed.abs() / 100.0));
            } else {
                settings.paused = true;
            }
        }
    }
}
