use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

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
        });
}

fn zoom_camera(
    mut query: Query<
        (&mut OrthographicProjection, &ActionState<CameraMovement>),
        With<Camera2d>,
    >,
) {
    const CAMERA_ZOOM_RATE: f32 = 0.05;

    let (mut camera_projection, action_state) = query.single_mut();
    let zoom_delta = action_state.value(CameraMovement::Zoom);
    camera_projection.scale *= zoom_delta.mul_add(-CAMERA_ZOOM_RATE, 1.0);
}
