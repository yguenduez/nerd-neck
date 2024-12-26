use ahrs::{Ahrs, Madgwick};
use nalgebra::{UnitQuaternion, Vector3};

struct MadgwickAdapter {
    madgwick: Madgwick<f32>,
}

impl MadgwickAdapter {
    pub fn new() -> Self {
        const SAMPLE_RATE: f32 = 0.1; // every 0.1 seconds
        const BETA: f32 = 0.05;
        MadgwickAdapter {
            madgwick: Madgwick::new(SAMPLE_RATE, BETA),
        }
    }
    pub fn update(&mut self, gyro: &Vector3<f32>, acc: &Vector3<f32>) -> &UnitQuaternion<f32> {
        self.madgwick.update_imu(gyro, acc).unwrap()
    }
}
