#![no_std]
#![no_main]

use embassy_executor::{task, Spawner};
use embassy_time::{Duration, Instant, Timer};
use esp_backtrace as _;
use esp_hal::gpio::{GpioPin, Level, Output};
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::{peripherals, prelude::*};
use log::info;
use test_esp32s3_embassy::ImuAdapter;
use utility::angle::{back_is_bend, quaternion_to_z_axis_angle};
use utility::madgwick_adapter::MadgwickAdapter;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
static SHARED: Signal<CriticalSectionRawMutex, NotifyPerson> = Signal::new();

struct NotifyPerson;

const POLL_INTERVAL: Duration = Duration::from_millis(20);

#[task]
async fn imu_poll(mut imu: ImuAdapter<'static>, mut madgwick: MadgwickAdapter) {
    loop {
        let (gyro, accel) = imu.get_data();
        let update_result = madgwick.update(gyro, accel);
        if let Ok(quaternion) = update_result {
            let (roll, pitch, yaw) = quaternion.euler_angles();
            info!("Roll: {:.2}, Pitch: {:.2}, Yaw: {:.2}", roll, pitch, yaw);

            let angle = quaternion_to_z_axis_angle((*quaternion).into());
            info!("Angle to z-axis: {:.2}", angle);

            // notify the other task if we surpass a certain threshold
            if back_is_bend(angle) {
                SHARED.signal(NotifyPerson);
            }
        }
        Timer::after(POLL_INTERVAL).await;
    }
}

#[task]
async fn notification(mut pin: Output<'static>) {
    loop {
        let _ = SHARED.wait().await;
        let now = Instant::now();
        loop {
            // TODO: Here we need to activate our beeper!
            pin.set_high();
            let duration = Duration::from_micros(500);
            Timer::after(duration).await;
            pin.set_low();

            // ignore new incoming signals for a given duration!
            let duration = Duration::from_micros(500);
            Timer::after(duration).await;
            let current_timestamp = Instant::now();
            let beeper_end = current_timestamp - now > Duration::from_secs(2);
            if beeper_end {
                break;
            }
        }
        SHARED.reset();
    }
}

#[main]
async fn main(spawner: Spawner) {
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
    let imu = ImuAdapter::new_bmi160(i2c);
    let madgwick = MadgwickAdapter::new(POLL_INTERVAL.as_millis());

    let gpio = peripherals.GPIO7;
    let beeperPin = Output::new(gpio, Level::Low);

    spawner.spawn(imu_poll(imu, madgwick)).unwrap();
    spawner.spawn(notification(beeperPin)).unwrap();
}
