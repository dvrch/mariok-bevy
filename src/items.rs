use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::player::Kart;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_item_boxes)
           .add_systems(Update, handle_item_collision);
    }
}

#[derive(Component)]
pub struct ItemBox;

fn spawn_item_boxes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let positions = vec![
        Vec3::new(30.0, 31.0, -80.0),
        Vec3::new(35.0, 31.0, -80.0),
        Vec3::new(40.0, 31.0, -80.0),
    ];

    let mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.0, 1.0, 1.0, 0.5),
        alpha_mode: AlphaMode::Blend,
        emissive: LinearRgba::rgb(0.0, 5.0, 5.0),
        ..default()
    });

    for pos in positions {
        commands.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(mat.clone()),
            Transform::from_translation(pos),
            Collider::cuboid(0.5, 0.5, 0.5),
            Sensor,
            ItemBox,
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
            let (item_ent, player_ent) = if item_query.contains(*e1) { (*e1, *e2) } 
                                        else if item_query.contains(*e2) { (*e2, *e1) } 
                                        else { continue; };

            if let Ok(mut kart) = player_query.get_mut(player_ent) {
                // Donne un boost immédiat pour le test
                kart.is_boosting = true;
                kart.boost_timer = 2.0;
                
                // Despawn l'item box
                commands.entity(item_ent).despawn_recursive();
                info!("Item collected! Boost activated!");
            }
        }
    }
}
