use nalgebra::{ComplexField, UnitQuaternion, Vector3};

pub fn quaternion_to_z_axis_angle(quat: UnitQuaternion<f32>) -> f32 {
    const Z_AXIS: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);
    let rotated_z = quat.transform_vector(&Z_AXIS);

    let dot_prod = rotated_z.dot(&Z_AXIS);
    let clamp = dot_prod.clamp(-1.0, 1.0);
    let angle = clamp.acos();
    angle
}

pub fn back_is_bend(angle: f32) -> bool {
    const THRESHOLD: f32 = core::f32::consts::PI / 8.0;
    ((core::f32::consts::PI / 2.0) - angle).abs() > THRESHOLD
}
