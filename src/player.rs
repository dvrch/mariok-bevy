use bevy::prelude::*;
use bevy::gltf::GltfAssetLabel;
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
           .add_systems(Update, (player_input, player_physics, player_reset, camera_follow));
    }
}

#[derive(Component)]
pub struct Kart {
    pub speed: f32,
    pub steering: f32,
    pub drift_dir: f32,
    pub drift_power: f32,
    pub is_boosting: bool,
    pub boost_timer: f32,
    pub jump_cooldown: f32,
    pub last_safe_pos: Vec3,
    pub last_safe_rot: Quat,
}

#[derive(Component)]
pub struct KartVisual;

#[derive(Component)]
pub struct FollowCamera;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let start_pos = Vec3::new(0.0, 2.0, 0.0);
    let start_rot = Quat::IDENTITY;

    // Main Physics Body
    commands.spawn((
        Transform::from_translation(start_pos).with_rotation(start_rot),
        RigidBody::Dynamic,
        Collider::ball(0.5),
        Damping { linear_damping: 0.5, angular_damping: 0.5 },
        ExternalImpulse::default(),
        Velocity::default(),
        Sleeping::disabled(),
        ActiveEvents::COLLISION_EVENTS,
        Kart {
            speed: 0.0,
            steering: 0.0,
            drift_dir: 0.0,
            drift_power: 0.0,
            is_boosting: false,
            boost_timer: 0.0,
            jump_cooldown: 0.0,
            last_safe_pos: start_pos,
            last_safe_rot: start_rot,
        },
        crate::logic::PlayerStats {
            current_lap: 1,
            last_checkpoint: -1,
            coin_count: 0,
        },
    )).with_children(|parent| {
        // Visual Model
        parent.spawn((
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/characters/mariokarttest.glb"))),
            Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::PI))
                .with_scale(Vec3::splat(0.01)),
            KartVisual,
        ));
    });

    // Camera initial target (Exactly at Mario's position as requested)
    let cam_start_pos = start_pos; 

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(cam_start_pos).looking_at(start_pos + Vec3::NEG_Z, Vec3::Y),
        FollowCamera,
    ));
}

fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Kart, &mut ExternalImpulse, &mut Velocity, &Transform)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (mut kart, mut impulse, _velocity, _transform) in query.iter_mut() {
        if kart.jump_cooldown > 0.0 { kart.jump_cooldown -= dt; }

        let up = keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::KeyZ);
        let down = keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS);
        let left = keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::KeyQ);
        let right = keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD);
        let jump = keyboard.pressed(KeyCode::KeyV) || keyboard.pressed(KeyCode::Space);

        let mut steer = 0.0;
        if left { steer += 1.0; }
        if right { steer -= 1.0; }
        kart.steering = steer;

        let mut acc = 0.0;
        if up { acc += 1.0; }
        if down { acc -= 1.0; }
        
        let max_speed = if kart.is_boosting { 65.0 } else { 38.0 };
        kart.speed = acc * max_speed;

        if jump && kart.jump_cooldown <= 0.0 {
            impulse.impulse += Vec3::Y * 4.5;
            kart.jump_cooldown = 0.7;
            if steer != 0.0 {
                kart.drift_dir = steer.signum();
            }
        }

        if !jump {
            if kart.drift_dir != 0.0 {
                if kart.drift_power > 0.8 {
                    kart.is_boosting = true;
                    kart.boost_timer = 1.2;
                }
                kart.drift_dir = 0.0;
                kart.drift_power = 0.0;
            }
        }
    }
}

fn player_physics(
    mut query: Query<(&mut Transform, &mut Velocity, &mut Kart, &mut ExternalImpulse)>,
    mut visual_query: Query<&mut Transform, (With<KartVisual>, Without<Kart>)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (mut transform, mut velocity, mut kart, mut impulse) in query.iter_mut() {
        // Rotation logic
        let mut target_rotation_speed = if kart.drift_dir != 0.0 {
            kart.drift_power += dt;
            kart.drift_dir * 3.8
        } else {
            kart.steering * 2.8
        };
        
        // Reduce rotation if moving slow
        let speed_factor = (velocity.linvel.length() / 10.0).min(1.0);
        target_rotation_speed *= speed_factor;
        
        transform.rotate_y(target_rotation_speed * dt);

        // Movement impulsions
        let forward = transform.forward();
        let current_forward_vel = velocity.linvel.dot(*forward);
        
        if kart.speed != 0.0 {
            let force = (kart.speed - current_forward_vel) * 4.5;
            impulse.impulse += forward * force * dt;
        }

        // Boost
        if kart.is_boosting {
            kart.boost_timer -= dt;
            impulse.impulse += forward * 50.0 * dt;
            if kart.boost_timer <= 0.0 { kart.is_boosting = false; }
        }

        // Stabilize angular velocity
        velocity.angvel.x = 0.0;
        velocity.angvel.z = 0.0;

        // Visual tilt & drift angle
        if let Some(mut visual_transform) = visual_query.iter_mut().next() {
            let drift_tilt = kart.drift_dir * 0.2;
            let steer_tilt = kart.steering * 0.1;
            visual_transform.rotation = Quat::from_rotation_y(std::f32::consts::PI + drift_tilt) * Quat::from_rotation_z(steer_tilt);
        }
        
        // Update safe position (simplified: if grounded)
        if transform.translation.y > 28.0 && transform.translation.y < 35.0 {
             kart.last_safe_pos = transform.translation;
             kart.last_safe_rot = transform.rotation;
        }
    }
}

fn player_reset(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity, &mut Kart)>,
) {
    for (mut transform, mut velocity, kart) in query.iter_mut() {
        if transform.translation.y < 5.0 || keyboard.just_pressed(KeyCode::KeyR) {
            transform.translation = kart.last_safe_pos + Vec3::Y * 2.0;
            transform.rotation = kart.last_safe_rot;
            velocity.linvel = Vec3::ZERO;
            velocity.angvel = Vec3::ZERO;
        }
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
            let target_pos = kart_transform.translation + *kart_transform.back() * 3.0 + Vec3::Y * 1.5;
            cam_transform.translation = cam_transform.translation.lerp(target_pos, 4.0 * dt);
            
            let look_at = kart_transform.translation + Vec3::Y * 0.8;
            cam_transform.look_at(look_at, Vec3::Y);
        }
    }
}
