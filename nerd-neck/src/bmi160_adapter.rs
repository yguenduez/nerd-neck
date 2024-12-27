use bmi160::interface::I2cInterface;
use bmi160::{AccelerometerPowerMode, Bmi160, GyroscopePowerMode, SensorSelector, SlaveAddr};
use esp_hal::Async;
use esp_hal::i2c::master::I2c;

pub struct ImuAdapter<'a> {
    inner: Bmi160<I2cInterface<I2c<'a, Async>>>,
}

impl<'a> ImuAdapter<'a>
{
    pub fn new_bmi160(i2c: I2c<'a, Async>) -> Self {
        let address = SlaveAddr::Alternative(true); //0x69
        let mut imu = Bmi160::new_with_i2c(i2c, address);
        imu.set_accel_power_mode(AccelerometerPowerMode::Normal)
            .unwrap();
        imu.set_gyro_power_mode(GyroscopePowerMode::Normal).unwrap();
        Self { inner: imu }
    }

    pub fn get_data(&mut self) -> ([f32; 3], [f32; 3]) {
        let data = self
            .inner
            .data_scaled(SensorSelector::new().accel().gyro())
            .unwrap();
        let accel = data.accel.unwrap();
        let gyro = data.gyro.unwrap();

        ([gyro.x, gyro.y, gyro.z], [accel.x, accel.y, accel.z])
    }
}
