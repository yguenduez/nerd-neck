# Nerd Neck on ESP32-S3

This small device will detect bad posture, if the person wearing this device has it attached to the back.

## Idea

We read IMU data in a cycle. Every x milliseconds we receive IMU data.
As the IMU values drift, we need a filter(s), that correct the drift integrated over time.

If the angle of the IMU passes a certain threshold and stays there for a certain amount of y seconds,
the device will start beeping, notifying the person to rectify its position.

The whole device is a wearable, so it needs a battery (LiPo) and a 3d printed casing.
We also try to make it low-powered.

## TODOs

- [x] Calculate an angle to Z-Axis (with Filters), that can be used as "beep" input
- [x] ~~Do a calibration for the Gyro before starting (if not moving, gyro values should be 0).~~ (Filter already corrects offsets)
- [ ] Write a little rust driver for the speaker, that controls it
- [x] On a configurable threshold - enable the trigger for the beep speaker
- [ ] Apply a LiPo (3.7 Volts) battery adapter to the battery pins
- [x] Design 3d model for the casing to print
- [x] Print the casing
- [ ] Assemble the parts
- [ ] Ongoing: Document everything nicely (can be done here)
- [ ] Test different parameters (Hz ratio, Beta value etc.), and document outcome

## IMU Drift

Gyroscope does drift, when integrated over time, so we need to correct those values with sensor fusion algorithms,
with other sensor values. Both IMUs we can use, also come with an accelerometer.
So we combine smooth short-them gyroscope data with long-term stability of accelerometer data.

Possible Filters:

- **Complementary filter** (simpler but effective for many use cases).
- **Madgwick filter** (a fast, quaternion-based algorithm for IMUs).
- **Mahony filter** (similar to Madgwick with some differences in accuracy and computation).
- **Kalman filter** (complex but precise, especially for combining multiple sensors).

**Note**: As we are only interested in the tilt of the IMU (diff angle to the vertical z-axis), we can neglect
the yaw angle (which would be corrected be the magnetometer, which we do not have).

So we can use either Madgwick or Mahony with only given accelerometer and gyro values.

### IMU Filter Implementation

There is already a [third party crate (ahrs-rs)](https://github.com/jmagnuson/ahrs-rs), that already implements both
Madgwick and Mahony filters
for both 6DoF and 9DoF IMUs.

A choice to be done is the filter gains, KI and KP for Mahony and Beta for Madgwick.

Some recommended beta values for the Madgwick Filter, where you have to choose
between responsiveness and stability:

| Drones/RC vehicles    | 0.1 - 0.3    | High dynamics, prioritize responsiveness.            |
|-----------------------|--------------|------------------------------------------------------|
| Robotics (general)    | 0.05 - 0.2   | Moderate dynamics, balance response and drift.       |
| Slow-motion systems   | 0.01 - 0.05  | Low dynamics, prioritize stability/noise rejection.  |
| Human motion tracking | 0.01 - 0.1   | Prioritize stability but allow small dynamic motion. |
| Static applications   | 0.001 - 0.01 | Low dynamics, focus on maximum stability.            |

TL;DR: We use the Madgwick filter with beta between 0.01 and 0.1. Currently, we use 0.1.

## IMU Sensors

We have several IMU Sensors

- [BMI160](https://www.dfrobot.com/product-1716.html)
- [MPU6050](https://learn.adafruit.com/mpu6050-6-dof-accelerometer-and-gyro/arduino)

Both can be addressed via a I2C-Bus.

## Software for the MPU6050

There is already a [MPU6050 crate](https://docs.rs/mpu6050/0.1.6/mpu6050/). It's usable with i2c.

## Software for the BMI160

There is already a [BMI160 create](https://docs.rs/bmi160/1.1.0/bmi160/). Also usable with i2c.

## I2C interface

This bus comes with a dataline (sda) and a line for the clock signal (scl).

You can connect several devices to the bus, like shown below.
![image](images/i2c_master_module.png)

Note from [espressif](https://docs.espressif.com/projects/esp-idf/en/v5.3.2/esp32s3/api-reference/peripherals/i2c.html):

> The frequency of SCL is influenced by both the pull-up resistor and the wire capacitance. Therefore, users are
> strongly recommended to choose appropriate pull-up resistors to make the frequency accurate. The recommended value for
> pull-up resistors usually ranges from 1K Ohms to 10K Ohms.
> Keep in mind that the higher the frequency, the smaller the pull-up resistor should be (but not less than 1 KOhms).
> Indeed, large resistors will decline the current, which will increase the clock switching time and reduce the
> frequency.
> We usually recommend a range of 2 KOhms to 5 KOhms, but users may also need to make some adjustments depending on
> their
> current draw requirements.

**TL;DR**: Use 2-10kOhm resistors as pull up resistors.
The higher the frequency, the lower the ohmage should be.

We use 4.7k Ohms as pull-up resistors.

## Pinouts of esp32s3 from Seeed

![Pinouts](images/pinouts-esp32.jpg)

## Current Wiring

We have an i2c setup, where two 4.7k Ohm resistors are used as
pull up resistors. If scl/sda are off, both signals are pulled up
up to the logical 3.3 Volts (high). The IMU (on the right)
will pull the signal down if needed.

Everything runs on the 3.3 Volts level of the MCU.

![fritzing](images/nerd-neck-fritzing.png)

## Building and flashing

### Prerequisites

Install all the depenendencies:

- Install [Rust](https://www.rust-lang.org/tools/install)
- Install esp tooling
    - `cargo install espup espflash`
    - `espup install`

### Building/Flashing

Then go into the `nerd-neck` directory and
`source ~/export-esp.h` (generated from `espup install`).

To build and flash it the firmware to the device,
just run `cargo run --release`.

## Casing

### Requirements

The casing must include

- a LiPo battery (3.7 Volts),
  e.g. [this one](https://www.bastelgarage.ch/lipo-akku-1500mah-jst-2-0-lithium-ion-polymer?search=lipo) (52x42x5mm),
- the BMI160 IMU from DFRobots with dimensions 23x27mm as well as the
- the esp32s3 from seeed with dimensions ~20x25mm. Also having access to its usb-c port.
- the piezo buzzer with its dimensions, e.g. [this one](https://www.bastelgarage.ch/piezo-buzzer-summer-aktiv) (12mm)

As the casing will be quite flat, we have on one layer the flat LiPo battery on the bottom
and on the other layer the IMU, the MCU and the piezo buzzer. The wiring will be in between
those two.

Also: It should be possible to exchange all parts.

The casing will have the dimension of 56x46x10mm, so we
have a will thickness of 1mm on each side.

### Version 0.1 

The first design looks like this: All parts have been designed
with freecad.

![assembly](images/assembly.png)

Where the parts are the following:

- Red: the IMU
- Grey: Esp32s3
- Yellow: Piezo speaker
- Blue: the LiPo battery

The casing itself consists of a bottom and a top casing.
The bottom casing houses the battery.
The top casing holds the rest. Both parts are designed that way, that the
bottom half slips into the top half and snaps in.

The whole casing has the following dimensions: 56x46x15mm (circle on top for the buzzer has hight of 6mm)

### Version 0.2

Changes to V0.1:
- To make the design more 3D-print friendly, there is no exterior for the buzzer any more.
  The Downside is,
the buzzer is exposed a bit.
- The hole for the usb got bigger, as it did not go through without drilling.
- The two holders, that hold the bottom part, got 1mm longer, as the bottom part did not fit in.

![assmbly](images/assembly_v0.2.png)

### Consequences for IMU

Building the imu like this, the IMU's X-Axis,
aligns with the global Z-Axis (direction of gravity).
Being completely "straight" means that the angle
between the Z-Axis of the IMU and the global Z-Axis
is PI/2.
This means our threshold for notifying
the person is between is a difference from PI/2.
The bigger difference, the higher the person "bends" it back.
