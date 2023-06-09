use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lerp::Lerp;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<CameraMovement>::default())
            .add_startup_system(spawn_camera)
            .add_system(zoom_camera);
    }
}

#[derive(Actionlike, Clone, Copy, Debug, PartialEq, Eq)]
enum CameraMovement {
    Zoom,
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(InputManagerBundle::<CameraMovement> {
            input_map: InputMap::default()
                .insert(SingleAxis::mouse_wheel_y(), CameraMovement::Zoom)
                .build(),
            ..Default::default()
        })
        .insert(TargetCameraScale(1.0));
}

/// The scale used by `zoom_camera()` to lerp the camera's scale to every frame.
#[derive(Component)]
struct TargetCameraScale(pub f32);

fn zoom_camera(
    mut query: Query<
        (
            &mut OrthographicProjection,
            &mut TargetCameraScale,
            &ActionState<CameraMovement>,
        ),
        With<Camera2d>,
    >,
) {
    // The value of the `Zoom` action will be 1 or -1 if the mouse wheel was moved
    // this frame or 0 if it wasn't. In other words, it's a delta value.
    // Stateless. To fix that, `TargetCameraScale` is used as the state that stores
    // what the camera scale should be lerped toward.
    const CAMERA_ZOOM_RATE: f32 = 0.1;
    const LERP_SPEED: f32 = 0.1;

    let (mut cam_projection, mut target_scale, action_state) = query.single_mut();
    let zoom_delta = action_state.value(CameraMovement::Zoom);

    // Update the target scale.
    target_scale.0 *= zoom_delta.mul_add(-CAMERA_ZOOM_RATE, 1.0);
    target_scale.0 = target_scale.0.clamp(1.0, f32::MAX);

    // Update the actual scale.
    cam_projection.scale = cam_projection.scale.lerp(target_scale.0, LERP_SPEED);
}
