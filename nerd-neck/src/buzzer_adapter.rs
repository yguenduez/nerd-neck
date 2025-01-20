use embassy_time::{Duration, Instant, Timer};
use esp_hal::gpio::Output;

pub struct BuzzerAdapter<'a> {
    pin: Output<'a>,
}

impl<'a> BuzzerAdapter<'a> {
    pub fn new(pin: Output<'a>) -> Self {
        Self { pin }
    }

    fn set_high(&mut self) {
        self.pin.set_high();
    }

    fn set_low(&mut self) {
        self.pin.set_low();
    }

    pub async fn beep_2_seconds(&mut self) {
        let now = Instant::now();
        loop {
            self.set_high();

            let duration = Duration::from_micros(500);
            Timer::after(duration).await;

            self.set_low();

            let duration = Duration::from_micros(500);
            Timer::after(duration).await;

            let current_timestamp = Instant::now();
            let beeper_end = current_timestamp - now > Duration::from_secs(2);
            if beeper_end {
                break;
            }
        }
    }
}
