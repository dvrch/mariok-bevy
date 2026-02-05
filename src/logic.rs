use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_checkpoints)
           .add_systems(Update, handle_checkpoint_collision);
    }
}

#[derive(Component)]
pub struct Checkpoint {
    pub index: usize,
}

#[derive(Component)]
pub struct PlayerStats {
    pub current_lap: usize,
    pub last_checkpoint: i32,
    pub coin_count: usize,
}

fn spawn_checkpoints(mut commands: Commands) {
    let positions = vec![
        Vec3::new(25.0, 30.0, -100.0),
        Vec3::new(100.0, 30.0, 0.0),
        Vec3::new(0.0, 30.0, 100.0),
        Vec3::new(-100.0, 30.0, 0.0),
    ];

    for (i, pos) in positions.into_iter().enumerate() {
        commands.spawn((
            Transform::from_translation(pos),
            Collider::cuboid(10.0, 10.0, 1.0),
            Sensor,
            Checkpoint { index: i },
            Name::new(format!("Checkpoint {}", i)),
        ));
    }
}

fn handle_checkpoint_collision(
    mut collision_events: EventReader<CollisionEvent>,
    checkpoint_query: Query<&Checkpoint>,
    mut player_query: Query<&mut PlayerStats>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let (checkpoint_ent, player_ent) = if checkpoint_query.contains(*e1) { (*e1, *e2) } 
                                              else if checkpoint_query.contains(*e2) { (*e2, *e1) } 
                                              else { continue; };

            if let Ok(checkpoint) = checkpoint_query.get(checkpoint_ent) {
                if let Ok(mut stats) = player_query.get_mut(player_ent) {
                    if checkpoint.index == 0 && stats.last_checkpoint == 3 {
                        stats.current_lap += 1;
                        info!("Lap {}!", stats.current_lap);
                    }
                    stats.last_checkpoint = checkpoint.index as i32;
                }
            }
        }
    }
}
