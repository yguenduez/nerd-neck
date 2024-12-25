#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use bmi160::{AccelerometerPowerMode, Bmi160, GyroscopePowerMode, SensorSelector, SlaveAddr};
use esp_backtrace as _;
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::prelude::*;
use esp_println::println;

#[main]
async fn main(_spawner: Spawner) -> ! {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_println::logger::init_logger_from_env();

    let timer0 = esp_hal::timer::systimer::SystemTimer::new(peripherals.SYSTIMER)
        .split::<esp_hal::timer::systimer::Target>();
    esp_hal_embassy::init(timer0.alarm0);

    // Getting the i2c up
    let i2c = I2c::new(peripherals.I2C0, Config::default())
        .with_scl(peripherals.GPIO6)
        .with_sda(peripherals.GPIO5).into_async();

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
