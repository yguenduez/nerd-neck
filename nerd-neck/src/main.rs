#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::prelude::*;
use esp_println::println;
use test_esp32s3_embassy::ImuAdapter;
use utility::angle::quaternion_to_z_axis_angle;
use utility::madgwick_adapter::MadgwickAdapter;

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
        .with_sda(peripherals.GPIO5)
        .into_async();

    //Setting up the IMU
    let mut imu = ImuAdapter::new_bmi160(i2c);
    let mut madgwick = MadgwickAdapter::new();
    // TODO: First we need to calibrate the gyro - starting the device on a desk
    // and not moving
    // then we loop in a defined rate
    loop {
        let (gyro, accel) = imu.get_data();
        let quaternion = madgwick.update(gyro, accel);

        let (roll, pitch, yaw) = quaternion.euler_angles();
        println!("Roll: {:.2}, Pitch: {:.2}, Yaw: {:.2}", roll, pitch, yaw);

        let angle = quaternion_to_z_axis_angle((*quaternion).into());
        println!("Angle to z-axis: {:.2}", angle);

        Timer::after(Duration::from_millis(20)).await;
    }
}
