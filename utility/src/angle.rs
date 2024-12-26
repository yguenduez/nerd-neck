use nalgebra::{UnitQuaternion, Vector3, ComplexField};

pub fn quaternion_to_z_axis_angle(quat: UnitQuaternion<f32>) -> f32 {
    const Z_AXIS: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);
    let rotated_z = quat.transform_vector(&Z_AXIS);

    let dot_prod = rotated_z.dot(&Z_AXIS);
    let clamp = dot_prod.clamp(-1.0, 1.0);
    let angle = clamp.acos();
    angle
}
