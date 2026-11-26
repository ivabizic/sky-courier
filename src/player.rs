use bevy::prelude::*;

use crate::camera::{DebugCameraState, FollowCamera};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct Speed(f32);

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    debug_camera: Res<DebugCameraState>,
    mut player_q: Query<(&mut Transform, &Speed), (With<Player>, Without<FollowCamera>)>,
    cam_q: Query<&Transform, (With<FollowCamera>, Without<Player>)>,
) {
    if debug_camera.enabled {
        return;
    }

    // Implementation for player movement
    for (mut player_transform, player_speed) in player_q.iter_mut() {
        let cam = match cam_q.single() {
            Ok(camera) => camera,
            Err(error) => {
                warn!("Expected exactly one follow camera: {error}");
                continue;
            }
        };

        let mut direction = Vec3::ZERO;

        //forward
        if keys.pressed(KeyCode::KeyW) {
            direction += *cam.forward();
        }

        //backward
        if keys.pressed(KeyCode::KeyS) {
            direction += *cam.back();
        }

        //left
        if keys.pressed(KeyCode::KeyA) {
            direction += *cam.left();
        }

        //right
        if keys.pressed(KeyCode::KeyD) {
            direction += *cam.right();
        }

        direction.y = 0.0; // Prevent vertical movement

        let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_secs();
        player_transform.translation += movement;
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Speed(2.0),
        Player,
    ));
}
