use bevy::prelude::*;
use bevy::gltf::GltfAssetLabel;
use bevy_rapier3d::prelude::*;

pub struct TrackPlugin;

impl Plugin for TrackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_track);
    }
}

fn spawn_track(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load the GLB track
    let track_handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/tracks/paris-bis.glb"));

    commands.spawn((
        SceneRoot(track_handle),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(0.5)), // Correspond à l'échelle héritée (50 * 0.01)
        AsyncCollider(ComputedColliderShape::TriMesh(TriMeshFlags::default())), // Génère le collider précis pour le circuit
        RigidBody::Fixed,
    ));
}
