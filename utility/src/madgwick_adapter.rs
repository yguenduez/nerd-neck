use ahrs::{Ahrs, Madgwick};
use nalgebra::{UnitQuaternion, Vector3};

pub struct MadgwickAdapter {
    madgwick: Madgwick<f32>,
}

impl MadgwickAdapter {
    pub fn new(poll_intervall_millis: u64) -> Self {
        let sample_rate: f32 = (poll_intervall_millis as f64 / 1000.0) as f32;
        const BETA: f32 = 0.1;
        MadgwickAdapter {
            madgwick: Madgwick::new(sample_rate, BETA),
        }
    }

    fn to_radians(&self, vec: &[f32; 3]) -> Vector3<f32> {
        const SCALE: f32 = core::f32::consts::PI / 180.0;
        Vector3::new(vec[0] * SCALE, vec[1] * SCALE, vec[2] * SCALE)
    }
    pub fn update(&mut self, gyro: [f32; 3], acc: [f32; 3]) -> &UnitQuaternion<f32> {
        self.madgwick
            .update_imu(
                &self.to_radians(&gyro),
                &Vector3::new(acc[0], acc[1], acc[2]),
            )
            .unwrap()
    }
}
