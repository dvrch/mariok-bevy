use bevy::prelude::*;
use crate::player::Kart;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
           .add_systems(Update, update_ui);
    }
}

#[derive(Component)]
struct HudText;

fn setup_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(""),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                HudText,
            ));
        });
}

fn update_ui(
    kart_query: Query<(&Kart, &crate::logic::PlayerStats)>,
    mut text_query: Query<(&mut Text, &mut TextColor), With<HudText>>,
) {
    if let Ok((kart, stats)) = kart_query.get_single() {
        if let Ok((mut text, mut color)) = text_query.get_single_mut() {
            text.0 = format!(
                "Lap: {} | Coins: {} | Speed: {:.0}",
                stats.current_lap,
                stats.coin_count,
                kart.current_speed.abs()
            );
            
            if kart.is_boosting {
                *color = TextColor(Color::srgb(1.0, 0.5, 0.0));
            } else {
                *color = TextColor(Color::WHITE);
            }
        }
    }
}
