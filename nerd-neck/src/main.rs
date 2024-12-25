#![no_std]
#![no_main]

use bmi160::{AccelerometerPowerMode, Bmi160, GyroscopePowerMode, SensorSelector, SlaveAddr};
use esp_backtrace as _;
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::prelude::*;
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Getting the i2c up
    let i2c = I2c::new(peripherals.I2C0, Config::default())
        .with_scl(peripherals.GPIO6)
        .with_sda(peripherals.GPIO5);

    //Setting up the IMU
    let address = SlaveAddr::Alternative(true); //0x69
    let mut imu = Bmi160::new_with_i2c(i2c, address);
    imu.set_accel_power_mode(AccelerometerPowerMode::Normal)
        .unwrap();
    imu.set_gyro_power_mode(GyroscopePowerMode::Normal).unwrap();
    loop {
        let data = imu.data(SensorSelector::new().accel().gyro()).unwrap();
        let accel = data.accel.unwrap();
        let gyro = data.gyro.unwrap();
        println!(
            "Accelerometer: x {:5} y {:5} z {:5}, \
             Gyroscope: x {:5} y {:5} z {:5}",
            accel.x, accel.y, accel.z, gyro.x, gyro.y, gyro.z
        );
    }
}
