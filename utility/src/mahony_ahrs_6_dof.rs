use nalgebra::{Quaternion, Vector3};

// source: https://x-io.co.uk/open-source-imu-and-ahrs-algorithms/
// port of mahony for IMU update (6DoF-without magnetometer)

struct MahonyAHRS6DOF {
    offset_acceleration: Option<[f32; 3]>,
    offset_gyroscope: Option<[f32; 3]>,
    current_orientation: Quaternion<f32>,
    integral_error_terms: [f32; 3],
    ki: u32,
    kp: u32,
}

impl MahonyAHRS6DOF {
    pub fn new() -> Self {
        const UP: Quaternion<f32> = Quaternion::new(1.0, 0.0, 0.0, 0.0);
        const ERROR_TERMS: [f32; 3] = [0f32, 0f32, 0f32];
        MahonyAHRS6DOF {
            offset_acceleration: None,
            offset_gyroscope: None,
            ki: 0,
            kp: 0,
            integral_error_terms: ERROR_TERMS,
            current_orientation: UP,
        }
    }

    pub fn current_orientation(&self) -> &Quaternion<f32> {
        &self.current_orientation
    }

    pub fn update(&mut self, acc: &[f32; 3], gyro: &mut [f32; 3], mut delta_t: f32) -> Quaternion<f32>{
        let qa = self.current_orientation.w;
        let qb = self.current_orientation.i;
        let qc = self.current_orientation.j;
        let qd = self.current_orientation.k;

        let acc_vec: Vector3<f32> = Vector3::new(acc[0], acc[1], acc[2]);
        if acc_vec.norm() > 0.0 {
            let vx = qb * qd - qa * qc;
            let vy = qa * qb + qc * qd;
            let vz = qa * qa - 0.5f32 + qd * qd;

            // Error is cross product between estimated and measured direction of gravity in body frame
            // (half the actual magnitude)
            let errors = [
                acc_vec.y * vz - acc_vec.z * vy,
                acc_vec.z * vx - acc_vec.x * vz,
                acc_vec.x * vy - acc_vec.y * vx,
            ];

            // Compute and apply to gyro term the integral feedback, if enabled
            if self.ki > 0 {
                // integral error scaled by ki
                for i in 0..3 {
                    self.integral_error_terms[i] += self.ki as f32 * errors[i] * delta_t;
                }
                // apply integral feedback
                for i in 0..3 {
                    gyro[i] += self.integral_error_terms[i];
                }
            }
        }

        // integrate orientation
        delta_t *= 0.5;
        gyro.iter_mut().for_each(|g| *g *= delta_t);

        self.current_orientation.w += -qb * gyro[0] - qc * gyro[1] - qd * gyro[2];
        self.current_orientation.i += qa * gyro[0] + qc * gyro[2] - qd * gyro[1];
        self.current_orientation.j += qa * gyro[1] - qb * gyro[2] + qd * gyro[0];
        self.current_orientation.k += qa * gyro[2] + qb * gyro[1] - qc * gyro[0];

        self.current_orientation.normalize_mut();
        self.current_orientation
    }
}
