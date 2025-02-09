# Nerd Neck - A wearable health device

## Context

This is the report for the nerd neck device. An embedded project for the lecture _Computer Architecture_ at the
University Basel for
the semester 2024.

Date: January 21st 2025

Student: Yasin Gündüz

# Abstract

In today's office work people are sitting much, which results in several health issues, like back pain
or chronicle bad postures. One of these is called "nerd neck", where the head protrudes forward from the shoulders.
This project is an effort to help people maintain a good posture while sitting
or standing, to decrease health issues originated by bad posture habits.

The nerd neck device is a 3d-printed wearable device where the device is
attached to a persons back and tracks the back's orientation in regards the gravity's axis.
Using an ESP32 and programmed in Rust, an inertial measurement unit together with a sensor fusion filter is
efficiently detecting the person's posture. When a bad posture is detected, a buzzer will notify the person to correct
the posture.

# Introduction

Office workers, or programmers, tend to have a bad posture in front of their laptops, or working at
desks. When exposed too long to a bad posture, this can result in several health issues. Back pain is just one example
of this.
There are already countermeasures to fight bad postures. For example ergonomic trainings for office
workers are carried out, to make working in office jobs healthier.

The approach this project is following is a bit different. Instead of proactive trainings,
the outcome of this project should be to react to a bad posture of a person and a user
should be notified when having a bad posture.

As this project should be wearable, it means the device must be battery powered. Also, as a person
is wearing this device, the form factor of the device should not be big, so it is for example
attachable to a person's back.

The project is already restricted to be as small as possible and battery powered. Being battery powered,
it also means it is not allowed to draw much current to remain active for as long as possible.

Moreover, it must track the posture of a person to be able to react to a bad posture.
The person must also be notified when being in a bad posture.

In the following sections, I will describe how I solved above goals.

# Methodology

The methodology is separated in a hardware section and a software section. In the hardware section,
we will look at the components, the wiring between them and the 3d printed casing, that houses them.
Afterwards, we will look at how software is designed and how it supplements the project goals to get
the orientation of the device.

## Hardware

### Components and Wiring

To be as small as possible, the esp32s3 from seeed xiao, a microcontroller unit (MCU), is used.
It comes with a small form factor and can be directly
soldered to a Lithium-Polymer (LiPo) battery for which it has an integrated circuit to load the battery.
This already solves the battery-driven goal of this project.

To detect the person's bad posture the orientation of a person's back is measured. Therefore,
we use an inertial measurement unit (IMU), that gives us the orientation information with its gyroscope values.
The IMU is connected via an inter-integrated-circuit (I2C), a common serial communication protocol,
to communicate with the MCU.

When reaching a certain orientation (i.e. a bad posture), an active buzzer is attached to one of the
general purpose output/input (GPIO) pin of the MCU.

Below, you can see the wiring of the breadboard version of the device. The I2C is connected to the GPIO pins
5 and 6. To make the I2C work, it is necessary to have pull-up resistors,
that pull the signal to the MCU's high logical voltage (3.3 volts). The IMU itself can only pull the signal down
to the logical zero volts.
This is what I had to learn during several hours of debugging. Common I2C resistor values of 4.7k Ohms are used.

The resistor for the buzzer is to protect the MCU's GPIO from overdrawing current of the buzzer. There a 100 Ohms
resistor is used.

<div align="center">
<img src="images/nerd-neck-fritzing.png" alt="Fritzing Image" width="500"/>
</div>

For a complete list of all the components, have a look at the reference section.

### 3d Printed Casing

As the device should be wearable, all the components from above need to be housed.
Therefore, a 3d printed casing was designed. It consists of a bottom shell and
top shell, that can be connected to one another, to close the housing.

As the LiPo battery is quite flat, the battery is housed in the bottom part, whereas the
top shell is housing the MCU, the buzzer as well as the IMU.

You can see both parts in the pictures below, without and with the components in it.

<div style="display: flex; justify-content: space-evenly; width: 100%;">
  <img src="images/case_empty_open.jpeg" alt="Fritzing Image" width="400"/>
  <img src="images/prototype_open.jpeg" alt="Fritzing Image" width="400"/>
</div>

There were a lot of problems with the initial designs (you can see them at
the [nerd-neck project](https://github.com/yguenduez/nerd-neck)).
First, it was tried
to create a design that allows the bottom shell to be snapped in the top shell of the housing to close
it.

However, the small snapping elements always broke because they have been too thin and therefore quite weak.
In a second iteration, it was tried to make the housing slideable. Meaning, we can slide the bottom (battery part) into
the top shell. With this design, the housing was just too big, compared to the inner volume the components are housed
in.

In the end, after already four design iterations, a small tip from a colleague helped. He mentioned that I should
use a so-called "pressfit" version. This means the two parts do not fit perfectly into one another, so you have to press
them into each other. The friction alone keeps both parts together. As an immediate consequence, the housing got much
simpler to design—and also to use.

# Software

The firmware for the esp32s3 is written with Rust. [Espressif](https://www.espressif.com/), the creators of the esp32
family created a lot of tooling in Rust
for their MCUs. With one line of a command, you can build and flash the firmware directly onto the esp32 from any host
system (in this case MacOS or Windows 11) via usb-c. This project makes also use of the
[esp_hal](https://github.com/esp-rs/esp-hal). A hardware abstraction layer for the esp32 family.

The only downside to Rust in the embedded world is that it is quite new. Tutorials and HowTos
were already outdated when reading them. Generally, you have to stick to the latest documentation,
that comes with a library you use.
Trying to use ChatGPT for programming was useless, as the recommended APIs were already outdated. And most
of the APIs had breaking changes in them.

## Architecture

In software, the tracking of a person's pose and the notification of a bad posture are separated. Therefore,
there are two concurrent running tasks, namely the IMU polling task and the notification task, which you can see
in the below flowchart diagram.

![flowchart](images/flowchart.png)

The IMU polling task gets the angular velocities,
as well as the acceleration data from the IMU every 50 milliseconds.
Directly after, both vectors are given to the Madgwick filter adapter, to correct the errors of the IMU
data.

From our filter we receive a quaternion, which describes the current orientation of our device.
At the end of the loop cycle of the IMU polling task, we check if the device's orientation to the z-Axis (direction of
gravity) surpasses a threshold.
Namely, if the angle between the device and the gravity's direction
is greater than 22.5 degrees, we send a signal to the notification task, to wake it up.

When the notification task is awakened, we generate a PWM signal of 1kHZ for 2 seconds and put the notification task
back to sleep to save power afterwards.

When the PWM signal is on, an active buzzer generates sound.
We manually generate the PWM signal
by setting a GPIO pin to high, wait 500 microseconds and set the GPIO pin
to low again. While the Buzzer is active, any further signals from the first task are ignored.

It is important to note that the asynchronous rust framework [embassy](https://embassy.dev/) for embedded
systems allows the tasks to be concurrent. On high level, it acts as a scheduler for our embedded device, putting
tasks to sleep, and waking them up, when needed.

## Sensor Fusion

### Choosing a Filter

IMU angular velocities cannot just be integrated over time, as those values
are error-prone. One will receive a so-called IMU-Drift, where the errors are also integrated over time.

To solve this issue, there are already several filters at hand, that could be
used for the project:

- Kalman Filter
- Mahony Filter
- Complementary Filter
- etc.

It has been a spoilt for choice. The filter chosen, usually used for an embedded device, is the Madgwick Filter.
Additionally, there has been an open-source library with its implementation at
hand ([ahrs-rs](https://github.com/jmagnuson/ahrs-rs)) that is used.

### Madgwick Filter

The Madgwick filter ([paper](https://x-io.co.uk/downloads/madgwick_internal_report.pdf)) is a sensor fusion algorithm
by Sebastian Madgwick,
that is suited well for embedded devices due to its efficiency.

It integrates the angular velocities, by integration of the quaternion derivatives over time.
As integration of angular velocities from an IMU is prone to drift, the madgwick filter uses
the earth's gravity field as a reference direction to compensate for the IMU
drift
([source here](https://ahrs.readthedocs.io/en/latest/filters/madgwick.html#orientation-as-solution-of-gradient-descent)).

The Madgwick filter has one parameter you need to adjust.
It is called the filter gain beta.

When picking the right value for beta, there is a trade-off between the stability
of the resulting orientation and its response. For example, for drones which
have to react fast, the Madgwick Filter is optimised for high responsiveness by using a high beta value.
On almost static, or human motion tracking, the filter is optimised for stable output using lower beta values.

A value of 0.1 is used (This is the default value chosen by the library that is used),
showed already a good trade-off between responsiveness and stability for the application.

# Closing

Doing this project was really rewarding. On the one hand, you could integrate
already known concepts from software engineering. On the other hand you could integrate
knowledge from hobby projects like 3d printing.
You could even learn something completely new, which was Rust on the embedded side and
soldering for me.

In the end, I created a battery-driven, wearable device, that can track a posture
and notify a person when having a bad posture and thus can help
people foster a healthier lifestyle.

Below the image of the assembled device

<div align="center">
<img src="images/prototype.png" alt="Finished prototype" width="400"/>
</div>

But of course, there are several points, that can be improved, which I describe in the following section.

## Improvements

The first thing, that can be improved, is the battery management. The MCU can load the battery,
but there is no information about the current charge of the battery. This is potentially dangerous,
as the LiPo could be drawn empty.

Also, being new at soldering, the cable management could be improved a lot. Maybe adapter solutions could be used
instead of soldering connections.

Furthermore, it would have been interesting to fine tune to the beta value of the Madgwick filter, which I did not do.

In the below list, more improvements are added:

- Using an MCU with an integrated IMU: For example,
  the [Seeed xiao nRF sense](https://www.seeedstudio.com/Seeed-XIAO-BLE-Sense-nRF52840-p-5253.html) comes with an
  integrated IMU. With this, the I2C connection could be made obsolete.
- Smaller LiPo Battery, making the device smaller: The battery is quite oversized for such a small project.
- The MCU supports Bluetooth. One could connect the device via bluetooth to a
  smartphone.

# References

The project itself is open-sourced at [https://github.com/yguenduez/nerd-neck](https://github.com/yguenduez/nerd-neck).
The code, stl files for printing and more documentation (on e.g. how to build and flash the project), can be found
there.

## Content

- [Madgwick Paper](https://courses.cs.washington.edu/courses/cse466/14au/labs/l4/madgwick_internal_report.pdf)
- [Sharing Data amongst asynchronous tasks in embassy](https://dev.to/theembeddedrustacean/sharing-data-among-tasks-in-rust-embassy-synchronization-primitives-59hk)
- [Discussion on Madgwick Beta Value](https://stackoverflow.com/a/47772311/7585591)
- [Documentation for the esp32s3 from seeed xiao](https://wiki.seeedstudio.com/xiao_esp32s3_getting_started/)
- [Rust library documentations: docs.rs](https://docs.rs/)

## Use of open-source libraries and frameworks

This project makes use of several open source libraries:

- The [esp_hal](https://github.com/esp-rs/esp-hal), an esp hardware abstraction layer for Rust.
- [Embassy](https://embassy.dev/) is used. An asynchronous runtime for embedded systems in Rust.
- For the inertial measurement unit (IMU), the [bmi160-rs](https://github.com/eldruin/bmi160-rs) is used.
- For the Madgwick filter, I used [ahrs-rs](https://github.com/jmagnuson/ahrs-rs).
- [nalgebra](https://github.com/dimforge/nalgebra) is used to work with quaternions, calculating angles.

## Bill of Materials

- a small microcontroller unit (MCU), an esp32s3 from xiao seeed, which has a small form
  factor. [Link](https://www.bastelgarage.ch/seeed-studio-xiao-esp32-s3-1-2809?search=esp32s3%20xiao%20seeed).
- an inertial measurement unit (IMU), that can measure acceleration and the angular velocity with a
  gyroscope. [Link](https://www.bastelgarage.ch/gravity-i2c-bmi160-6-axis-motion-sensor-with-gyroscope?search=bmi160)
- a small lithium polymer (LiPo) battery, to power the wearable
  device. [Link](https://www.bastelgarage.ch/solar-lipo-1-105/lipo-battery-1500mah-jst-2-0-lithium-ion-polymer).
- a small beeper/buzzer to notify the person about a bad
  posture. [Link](https://www.bastelgarage.ch/piezo-buzzer-summer-active?search=active%20buzzer)
- 2x 4.7k Ohm Resistors for the i2c connection.
- 1x 100 Ohm Resistor for the GPIO pin 7 for overdrawing protection
- Wires to connect the components by soldering
- JST-PH crimp plugs and sockets (To not directly solder the battery to the
  MCU), [link](https://www.bastelgarage.ch/jst-ph-crimp-stecker-und-buchsen-2mm-set-40-stuck)

# Disclaimer

OpenAI GPT-4o was trying to be used at the beginning of the project. But no generated code was used, as
the suggested API calls either did not exist (were hallucinated), or were just completely outdated.
In the end, library documentations at [docs.rs](https://docs.rs/) were the only valuable source of truth.