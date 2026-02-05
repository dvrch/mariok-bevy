use bevy::prelude::*;
use crate::player::Kart;
use crate::logic::PlayerStats;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
           .add_systems(Update, update_ui);
    }
}

#[derive(Component)]
struct HudText;

#[derive(Component)]
struct ItemIcon;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(30.0)),
                ..default()
            },
        ))
        .with_children(|parent| {
            // Stats (Left)
            parent.spawn((
                Text::new(""),
                TextFont {
                    font_size: 45.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                HudText,
            ));

            // Item Box (Right)
            parent.spawn((
                Node {
                    width: Val::Px(120.0),
                    height: Val::Px(120.0),
                    border: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                BorderColor(Color::srgb(0.8, 0.8, 0.0)),
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            )).with_children(|box_parent| {
                box_parent.spawn((
                    ImageNode::new(asset_server.load("images/mushroom.png")), // Hidden placeholder
                    Node {
                        width: Val::Percent(90.0),
                        height: Val::Percent(90.0),
                        display: Display::None, // Hide by default
                        ..default()
                    },
                    ItemIcon,
                ));
            });
        });
}

fn update_ui(
    kart_query: Query<(&Kart, &PlayerStats)>,
    mut text_query: Query<(&mut Text, &mut TextColor), With<HudText>>,
    mut icon_query: Query<(&mut Node, &mut ImageNode), With<ItemIcon>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((kart, stats)) = kart_query.get_single() {
        if let Ok((mut text, mut color)) = text_query.get_single_mut() {
            text.0 = format!(
                "LAP: {}/3\nCOINS: {}\nSPEED: {:.0} KM/H",
                stats.current_lap,
                stats.coin_count,
                (velocity_to_kmh(kart.speed)).abs()
            );
            
            if kart.is_boosting {
                *color = TextColor(Color::srgb(1.0, 0.6, 0.0));
            } else {
                *color = TextColor(Color::WHITE);
            }
        }

        // Item Icon logic
        if let Ok((mut node, mut image)) = icon_query.get_single_mut() {
            if kart.is_boosting { // Visual hack for now: show mushroom when boosting
                node.display = Display::Flex;
                image.image = asset_server.load("images/mushroom.png");
            } else {
                node.display = Display::None;
            }
        }
    }
}

fn velocity_to_kmh(vel: f32) -> f32 {
    vel * 3.6 / 10.0 // Scaled for better feeling
}
