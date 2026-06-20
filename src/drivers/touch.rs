use esp_hal::{
    gpio::{Input, InputConfig, Pull},
    time::{Duration, Instant},
};

const LONG_PRESS_MS: u64 = 800;
const DEBOUNCE_MS: u64 = 50;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TouchEvent {
    Tap,
    LongPress,
    None,
}

pub struct TouchSensor<'d> {
    pin: Input<'d>,
    last_state: bool,
    press_start: Option<Instant>,
}

impl<'d> TouchSensor<'d> {
    pub fn new(pin: impl esp_hal::gpio::InputPin + 'd) -> Self {
        let config = InputConfig::default().with_pull(Pull::Down);
        Self {
            pin: Input::new(pin, config),
            last_state: false,
            press_start: None,
        }
    }

    pub fn poll(&mut self) -> TouchEvent {
        let current = self.pin.is_high();

        if current && !self.last_state {
            self.press_start = Some(Instant::now());
        }

        if !current && self.last_state {
            if let Some(start) = self.press_start.take() {
                let held = start.elapsed();
                if held >= Duration::from_millis(DEBOUNCE_MS) {
                    self.last_state = current;
                    return if held >= Duration::from_millis(LONG_PRESS_MS) {
                        TouchEvent::LongPress
                    } else {
                        TouchEvent::Tap
                    };
                }
            }
        }

        self.last_state = current;
        TouchEvent::None
    }

    pub fn is_touched(&self) -> bool {
        self.pin.is_high()
    }
}
