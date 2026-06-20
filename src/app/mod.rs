pub mod state;

use defmt::{info, warn};
use esp_hal::time::{Duration, Instant};
use heapless::String;

use crate::{
    drivers::{
        battery::Battery,
        buzzer::Buzzer,
        oled::Oled,
        touch::{TouchEvent, TouchSensor},
    },
    storage::logger::{EventKind, EventLogger},
    ui::menu::{HomeMenuItem, home_lines},
};

use state::AppState;

const ALARM_HZ: u32 = 2_000;
const BEEP_MS: u64 = 200;
const PAUSE_MS: u64 = 150;
const BEEP_COUNT: u32 = 3;

pub struct App<'d, I2C> {
    oled: Oled<I2C>,
    touch: TouchSensor<'d>,
    buzzer: Buzzer<'d>,
    battery: Battery,
    logger: EventLogger,
    state: AppState,
    home_cursor: HomeMenuItem,
}

impl<'d, I2C> App<'d, I2C>
where
    I2C: embedded_hal::i2c::I2c,
{
    pub fn new(
        oled: Oled<I2C>,
        touch: TouchSensor<'d>,
        buzzer: Buzzer<'d>,
        battery: Battery,
    ) -> Self {
        info!("initialised");
        Self {
            oled,
            touch,
            buzzer,
            battery,
            logger: EventLogger::new(),
            state: AppState::Boot,
            home_cursor: HomeMenuItem::Arm,
        }
    }

    pub fn run(&mut self) -> ! {
        let boot_start = Instant::now();

        loop {
            match self.state {
                AppState::Boot => self.state_boot(boot_start),
                AppState::Home => self.state_home(),
                AppState::Armed => self.state_armed(),
                AppState::Alarm => self.state_alarm(),
                AppState::History => self.state_history(),
                AppState::Battery => self.state_battery(),
            }
        }
    }

    fn state_boot(&mut self, boot_start: Instant) {
        info!("State: Boot");
        self.oled.draw_lines(&["Initialising..."]);
        while boot_start.elapsed() < Duration::from_millis(2_000) {}
        self.state = AppState::Home;
    }

    fn state_home(&mut self) {
        let lines = home_lines(self.home_cursor);
        self.oled.draw_lines(&lines);

        match self.touch.poll() {
            TouchEvent::Tap => {
                self.home_cursor = self.home_cursor.next();
            }
            TouchEvent::LongPress => match self.home_cursor {
                HomeMenuItem::Arm => {
                    info!("Arming device");
                    self.logger.log(EventKind::Armed, 0);
                    self.state = AppState::Armed;
                }
                HomeMenuItem::History => {
                    info!("Opening history");
                    self.state = AppState::History;
                }
                HomeMenuItem::Battery => {
                    info!("Opening battery");
                    self.state = AppState::Battery;
                }
            },
            TouchEvent::None => {}
        }
    }

    fn state_armed(&mut self) {
        self.oled
            .draw_lines(&["ARMED", "", "Monitoring...", "Hold: disarm"]);

        match self.touch.poll() {
            TouchEvent::LongPress => {
                info!("Disarmed by user");
                self.logger.log(EventKind::Disarmed, 0);
                self.state = AppState::Home;
            }
            TouchEvent::Tap | TouchEvent::None => {
                if self.touch.is_touched() {
                    info!("triggering alarm");
                    self.logger.log(EventKind::AlarmTriggered, 0);
                    self.state = AppState::Alarm;
                }
            }
        }
    }

    fn state_alarm(&mut self) {
        warn!("State: Alarm");
        self.oled
            .draw_lines(&["! ALARM !", "", "Event saved", "Hold: dismiss"]);

        for _ in 0..BEEP_COUNT {
            self.buzzer.beep(ALARM_HZ, BEEP_MS);
            busy_wait_ms(PAUSE_MS);
        }

        loop {
            if self.touch.poll() == TouchEvent::LongPress {
                info!("Alarm dismissed");
                break;
            }
        }
        self.state = AppState::Home;
    }

    fn state_history(&mut self) {
        let count = self.logger.count();
        info!("History: {} events logged", count);
        let mut summary: String<32> = String::new();
        let _ = core::fmt::write(&mut summary, format_args!("Events: {}", count));

        self.oled
            .draw_lines(&["History", summary.as_str(), "", "Tap: back"]);

        loop {
            if self.touch.poll() != TouchEvent::None {
                break;
            }
        }
        self.state = AppState::Home;
    }

    fn state_battery(&mut self) {
        let pct = self.battery.percent();
        info!("Battery: {}%", pct);
        let mut line: String<16> = String::new();
        let _ = core::fmt::write(&mut line, format_args!("{}%", pct));

        self.oled
            .draw_lines(&["Battery", line.as_str(), "", "Tap: back"]);

        loop {
            if self.touch.poll() != TouchEvent::None {
                break;
            }
        }
        self.state = AppState::Home;
    }
}

fn busy_wait_ms(ms: u64) {
    let start = Instant::now();
    while start.elapsed() < Duration::from_millis(ms) {}
}
