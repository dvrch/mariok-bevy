use bevy::prelude::*;
use bevy::gltf::GltfAssetLabel;
use bevy_rapier3d::prelude::*;
use crate::player::Kart;
use crate::logic::PlayerStats;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_gameplay_objects)
           .add_systems(Update, (handle_item_collision, handle_coin_collision, animate_objects));
    }
}

#[derive(Component)]
pub struct ItemBox;

#[derive(Component)]
pub struct Coin;

#[derive(Component)]
struct Rotating;

fn spawn_gameplay_objects(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Porting positions from a typical Mario Kart layout or original project observation
    let box_positions = vec![
        Vec3::new(30.0, 31.0, -80.0),
        Vec3::new(35.0, 31.0, -80.0),
        Vec3::new(40.0, 31.0, -80.0),
        Vec3::new(-30.0, 31.0, 50.0),
        Vec3::new(-35.0, 31.0, 55.0),
    ];

    let coin_positions = vec![
        Vec3::new(20.0, 31.0, -90.0),
        Vec3::new(22.0, 31.0, -95.0),
        Vec3::new(24.0, 31.0, -100.0),
        Vec3::new(50.0, 31.0, 20.0),
        Vec3::new(55.0, 31.0, 25.0),
    ];

    // Spawn Item Boxes
    for pos in box_positions {
        commands.spawn((
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/misc/mario_kart_item_box.glb"))),
            Transform::from_translation(pos),
            Collider::cuboid(0.8, 0.8, 0.8),
            Sensor,
            ItemBox,
            Rotating,
        ));
    }

    // Spawn Coins
    for pos in coin_positions {
        commands.spawn((
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/misc/super_mario_bros_coin.glb"))),
            Transform::from_translation(pos),
            Collider::ball(0.5),
            Sensor,
            Coin,
            Rotating,
        ));
    }
}

fn handle_item_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    item_query: Query<Entity, With<ItemBox>>,
    mut player_query: Query<&mut Kart>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let (item_ent, _) = if item_query.contains(*e1) { (*e1, *e2) } 
                                        else if item_query.contains(*e2) { (*e2, *e1) } 
                                        else { continue; };

            if let Ok(mut kart) = player_query.get_single_mut() {
                // Random item logic (simplified)
                kart.is_boosting = true;
                kart.boost_timer = 2.0;
                
                commands.entity(item_ent).despawn_recursive();
                info!("Item collected! Boost activated!");
            }
        }
    }
}

fn handle_coin_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    coin_query: Query<Entity, With<Coin>>,
    mut player_query: Query<&mut PlayerStats>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let (coin_ent, _) = if coin_query.contains(*e1) { (*e1, *e2) } 
                                        else if coin_query.contains(*e2) { (*e2, *e1) } 
                                        else { continue; };

            if let Ok(mut stats) = player_query.get_single_mut() {
                stats.coin_count += 1;
                commands.entity(coin_ent).despawn_recursive();
                info!("Coin collected! Total: {}", stats.coin_count);
            }
        }
    }
}

fn animate_objects(mut query: Query<&mut Transform, With<Rotating>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_y(2.0 * time.delta_secs());
        // Subtle hover
        transform.translation.y += (time.elapsed_secs() * 2.0).sin() * 0.005;
    }
}
