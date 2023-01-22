use crate::prelude::*;

pub fn camera_shake(
    mut camera_shake_resource: ResMut<CameraShakeResource>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera2d>>,
) {
    let mut camera_transform = query.single_mut();
    if camera_shake_resource.0 > f32::EPSILON {
        camera_transform.translation.y = f32::sin(time.elapsed_seconds() * 80.0) * 4.0;
        camera_transform.translation.x = f32::cos(time.elapsed_seconds() * 80.0) * 4.0;
        camera_shake_resource.0 -= time.delta_seconds();
    } else {
        camera_transform.translation = Vec3::ZERO;
    }
}
