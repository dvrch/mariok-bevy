use bevy::prelude::*;
use bevy::gltf::GltfAssetLabel;
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
           .add_systems(Update, (player_physics, camera_follow));
    }
}

#[derive(Component)]
pub struct Kart {
    pub current_speed: f32,
    pub steering_speed: f32,
    pub drift_direction: f32, // 1 for left, -1 for right, 0 for none
    pub accumulated_drift_power: f32,
    pub is_boosting: bool,
    pub boost_timer: f32,
}

#[derive(Component)]
pub struct FollowCamera;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Kart Entity
    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/mario.glb"))),
        Transform::from_xyz(25.0, 30.0, -120.0),
        RigidBody::Dynamic,
        Collider::ball(0.5),
        Damping { linear_damping: 0.5, angular_damping: 0.5 },
        Restitution::coefficient(0.1),
        Friction::coefficient(0.5),
        ExternalImpulse::default(),
        Velocity::default(),
        Sleeping::disabled(),
        Kart {
            current_speed: 0.0,
            steering_speed: 0.02,
            drift_direction: 0.0,
            accumulated_drift_power: 0.0,
            is_boosting: false,
            boost_timer: 0.0,
        },
        crate::logic::PlayerStats {
            current_lap: 1,
            last_checkpoint: -1,
            coin_count: 0,
        },
    ));

    // Camera Entity
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 3.0, 8.0), // Offset original
        FollowCamera,
    ));
}

fn player_physics(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity, &mut Kart, &mut ExternalImpulse)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    for (mut transform, mut velocity, mut kart, mut impulse) in query.iter_mut() {
        // Boost Logic
        if kart.is_boosting {
            kart.boost_timer -= dt;
            if kart.boost_timer <= 0.0 {
                kart.is_boosting = false;
            }
        }

        // Steering
        let mut steer_input: f32 = 0.0;
        if keyboard.pressed(KeyCode::ArrowLeft) { steer_input += 1.0; }
        if keyboard.pressed(KeyCode::ArrowRight) { steer_input -= 1.0; }

        // Drifting Logic (Simplifiée pour Bevy)
        let jump_pressed = keyboard.pressed(KeyCode::KeyV); // Map 'V' as Jump/Drift
        if jump_pressed && steer_input != 0.0 && kart.drift_direction == 0.0 {
            kart.drift_direction = steer_input.signum();
        } else if !jump_pressed {
            if kart.drift_direction != 0.0 {
                // Release boost if enough power
                if kart.accumulated_drift_power > 1.0 {
                    kart.is_boosting = true;
                    kart.boost_timer = 1.0;
                }
            }
            kart.drift_direction = 0.0;
            kart.accumulated_drift_power = 0.0;
        }

        if kart.drift_direction != 0.0 {
            kart.accumulated_drift_power += dt;
            transform.rotate_y(kart.drift_direction * 150.0 * kart.steering_speed * dt);
        } else {
            transform.rotate_y(steer_input * 100.0 * kart.steering_speed * dt);
        }

        // Acceleration
        let mut forward_input = 0.0;
        if keyboard.pressed(KeyCode::ArrowUp) { forward_input += 1.0; }
        if keyboard.pressed(KeyCode::ArrowDown) { forward_input -= 1.0; }

        let base_speed = if kart.is_boosting { 60.0 } else { 30.0 };
        kart.current_speed = forward_input * base_speed;

        let forward = transform.forward();
        impulse.impulse = (forward * kart.current_speed * 10.0 * dt).into();

        // Prevent flipping
        velocity.angvel = Vec3::new(0.0, velocity.angvel.y, 0.0).into();
    }
}

fn camera_follow(
    kart_query: Query<&Transform, (With<Kart>, Without<FollowCamera>)>,
    mut cam_query: Query<&mut Transform, With<FollowCamera>>,
    time: Res<Time>,
) {
    if let Ok(kart_transform) = kart_query.get_single() {
        if let Ok(mut cam_transform) = cam_query.get_single_mut() {
            let dt = time.delta_secs();
            
            // Target position: behind and above the kart
            let target_pos = kart_transform.translation + *kart_transform.back() * 8.0 + Vec3::Y * 3.0;
            
            // Smooth lerping
            cam_transform.translation = cam_transform.translation.lerp(target_pos, 5.0 * dt);
            
            // Look at kart
            let look_target = kart_transform.translation + Vec3::Y * 1.0;
            cam_transform.look_at(look_target, Vec3::Y);
        }
    }
}
