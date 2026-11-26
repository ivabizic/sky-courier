use bevy::{
    input::mouse::{
        AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollPixelsPerLine,
},
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

use crate::player::Player;

const MOUSE_SENSITIVITY: f32 = 0.002;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .init_resource::<DebugCameraState>()
            .add_systems(
                Update,
                (toggle_debug_camera, follow_player_camera, move_debug_camera).chain(),
            );
    }
}

//stores which camera mode is active and the angles for each mode
#[derive(Resource)]
pub struct DebugCameraState {
    pub enabled: bool,
    follow_yaw: f32,
    follow_pitch: f32,
    follow_distance: f32,
    debug_yaw: f32,
    debug_pitch: f32,
}

impl Default for DebugCameraState {
    fn default() -> Self {
        Self {
            enabled: false,
            follow_yaw: 0.0,
            follow_pitch: 0.45,
            debug_yaw: 0.0,
            debug_pitch: 0.0,
            follow_distance: 10.0,
        }
    }
}

#[derive(Component)]
pub struct FollowCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        FollowCamera,
    ));
}

// F3 switches modes. Escape leaves debug mode.
fn toggle_debug_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<DebugCameraState>,
    camera_q: Query<&Transform, With<FollowCamera>>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    if keys.just_pressed(KeyCode::F3) {
        state.enabled = !state.enabled;

        if state.enabled {
            if let Ok(camera) = camera_q.single() {
                (state.debug_yaw, state.debug_pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);
            }
            lock_cursor(&mut cursor_options);
        } else {
            unlock_cursor(&mut cursor_options);
        }
    }

    if state.enabled && keys.just_pressed(KeyCode::Escape) {
        state.enabled = false;
        unlock_cursor(&mut cursor_options);
    }
}

fn follow_player_camera(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    scroll_conversion: Res<MouseScrollPixelsPerLine>,
    mut state: ResMut<DebugCameraState>,
    player_q: Query<&Transform, (With<Player>, Without<FollowCamera>)>,
    mut camera_q: Query<&mut Transform, (With<FollowCamera>, Without<Player>)>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    if state.enabled {
        return;
    }

    let scroll_y = mouse_scroll.to_lines(&scroll_conversion).delta.y;
    state.follow_distance = (state.follow_distance - scroll_y * 0.5).clamp(3.0, 20.0);

    // right-click to move camera
    if mouse_buttons.just_pressed(MouseButton::Right) {
        lock_cursor(&mut cursor_options);
    }
    if mouse_buttons.just_released(MouseButton::Right) {
        unlock_cursor(&mut cursor_options);
    }

    if mouse_buttons.pressed(MouseButton::Right) {
        state.follow_yaw += mouse_motion.delta.x * MOUSE_SENSITIVITY;
        state.follow_pitch =
            (state.follow_pitch - mouse_motion.delta.y * MOUSE_SENSITIVITY).clamp(-1.2, 1.2);
    }

    let Ok(player) = player_q.single() else {
        return;
    };

    let target = player.translation + Vec3::Y;
    let distance = state.follow_distance;
    let flat_distance = distance * state.follow_pitch.cos();
    let camera_offset = Vec3::new(
        state.follow_yaw.sin() * flat_distance,
        distance * state.follow_pitch.sin(),
        state.follow_yaw.cos() * flat_distance,
    );

    for mut camera in &mut camera_q {
        camera.translation = target + camera_offset;
        camera.look_at(target, Vec3::Y);
    }
}

//debug mode: WASD flies, Space/Ctrl goes up or down, Shift goes faster, mouse looks around
fn move_debug_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
    mut state: ResMut<DebugCameraState>,
    mut camera_q: Query<&mut Transform, With<FollowCamera>>,
) {
    if !state.enabled {
        return;
    }

    let Ok(mut camera) = camera_q.single_mut() else {
        return;
    };

    // Mouse controls the debug camera
    state.debug_yaw -= mouse_motion.delta.x * MOUSE_SENSITIVITY;
    state.debug_pitch =
        (state.debug_pitch - mouse_motion.delta.y * MOUSE_SENSITIVITY).clamp(-1.54, 1.54);
    camera.rotation = Quat::from_euler(EulerRot::YXZ, state.debug_yaw, state.debug_pitch, 0.0);

    // Add up directions
    let mut direction = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        direction += *camera.forward();
    }
    if keys.pressed(KeyCode::KeyS) {
        direction += *camera.back();
    }
    if keys.pressed(KeyCode::KeyA) {
        direction += *camera.left();
    }
    if keys.pressed(KeyCode::KeyD) {
        direction += *camera.right();
    }
    if keys.pressed(KeyCode::Space) {
        direction += Vec3::Y;
    }
    if keys.pressed(KeyCode::ControlLeft) {
        direction -= Vec3::Y;
    }

    let speed = if keys.pressed(KeyCode::ShiftLeft) {
        18.0
    } else {
        6.0
    };

    camera.translation += direction.normalize_or_zero() * speed * time.delta_secs();
}

fn lock_cursor(cursor_options: &mut CursorOptions) {
    cursor_options.visible = false;
    cursor_options.grab_mode = CursorGrabMode::Locked;
}

fn unlock_cursor(cursor_options: &mut CursorOptions) {
    cursor_options.visible = true;
    cursor_options.grab_mode = CursorGrabMode::None;
}
