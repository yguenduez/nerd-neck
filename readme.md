# Nerd Neck on ESP32-S3

We have several IMU Sensors
- [BMI160](https://www.dfrobot.com/product-1716.html)
- [MPU6050](https://learn.adafruit.com/mpu6050-6-dof-accelerometer-and-gyro/arduino)

Both can be addressed via a I2C-Bus.

## Software for the MPU6050

There is already a [MPU6050 crate](https://docs.rs/mpu6050/0.1.6/mpu6050/). It's usable with i2c.

## Software for the BMI160

There is already a [BMI160 create](https://docs.rs/bmi160/1.1.0/bmi160/). Also usable with i2c.

Below is an example how you can use it:

```rust
extern crate linux_embedded_hal as hal;
use bmi160::{AccelerometerPowerMode, Bmi160, GyroscopePowerMode, SensorSelector, SlaveAddr};

fn main() {
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut imu = Bmi160::new_with_i2c(dev, address);
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
```

## I2C interface

This bus comes with a dataline (sda) and a line for the clock signal (scl).

You can connect several devices to the bus, like shown below.
![image](images/i2c_master_module.png)

Below is a code snippet to connect the esp32 with i2c to an external device

```rust
use esp32_hal::{
    i2c::I2C,
    peripherals::Peripherals,
    prelude::*,
    clock::Clocks,
};
use embedded_hal::blocking::i2c::{Write, WriteRead};

fn main() {
    // Obtain the peripherals from the ESP32
    let peripherals = Peripherals::take().unwrap();

    // Configure the system clocks
    let clocks = Clocks::configure(
        peripherals.SYSTEM.clock_control,
        peripherals.SYSTEM.pll,
        peripherals.SYSTIMER,
    ).freeze();

    // Setup I2C pins
    let sda = peripherals.GPIO5.into_open_drain_gpio();
    let scl = peripherals.GPIO6.into_open_drain_gpio();

    // Initialize I2C peripheral
    let mut i2c = I2C::new(
        peripherals.I2C0,
        sda,
        scl,
        400.kHz(), // Set frequency
        &clocks,
    );

    // Use this `i2c` object with your driver
}
```

Note from [espressif](https://docs.espressif.com/projects/esp-idf/en/v5.3.2/esp32s3/api-reference/peripherals/i2c.html):

> The frequency of SCL is influenced by both the pull-up resistor and the wire capacitance. Therefore, users are strongly recommended to choose appropriate pull-up resistors to make the frequency accurate. The recommended value for pull-up resistors usually ranges from 1K Ohms to 10K Ohms.
Keep in mind that the higher the frequency, the smaller the pull-up resistor should be (but not less than 1 KOhms). Indeed, large resistors will decline the current, which will increase the clock switching time and reduce the frequency. We usually recommend a range of 2 KOhms to 5 KOhms, but users may also need to make some adjustments depending on their current draw requirements.

**TL;DR**: Use 2-10kOhm resistors as pull up resistors.
The higher the frequency, the lower the ohmage should be.

## Cargo Vendor

To use vendored sources, add this to your .cargo/config.toml for this project:

```
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
```