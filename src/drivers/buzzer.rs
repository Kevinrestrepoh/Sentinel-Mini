use esp_hal::{
    gpio::{Level, Output, OutputConfig},
    time::{Duration, Instant},
};

pub struct Buzzer<'d> {
    pin: Output<'d>,
}

impl<'d> Buzzer<'d> {
    pub fn new(pin: impl esp_hal::gpio::OutputPin + 'd) -> Self {
        let output = Output::new(pin, Level::Low, OutputConfig::default());
        Self { pin: output }
    }

    pub fn beep(&mut self, frequency_hz: u32, duration_ms: u64) {
        let half_period_us = 1_000_000 / (frequency_hz as u64 * 2);
        let deadline = Instant::now() + Duration::from_millis(duration_ms);

        while Instant::now() < deadline {
            self.pin.set_high();
            busy_wait_us(half_period_us);
            self.pin.set_low();
            busy_wait_us(half_period_us);
        }
    }

    pub fn stop(&mut self) {
        self.pin.set_low();
    }
}

fn busy_wait_us(us: u64) {
    let start = Instant::now();
    while start.elapsed() < Duration::from_micros(us) {}
}
