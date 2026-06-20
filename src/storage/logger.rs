use defmt::warn;
use esp_nvs::{Key, Nvs};
use esp_storage::FlashStorage;
use heapless::Vec;

const NVS_OFFSET: usize = 0x9000;
const NVS_SIZE: usize = 0x6000;

const NS: Key = Key::from_slice(b"log");
const KEY_COUNT: Key = Key::from_slice(b"cnt");
const MAX_EVENTS: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, defmt::Format)]
pub enum EventKind {
    Armed,
    Disarmed,
    AlarmTriggered,
}

impl EventKind {
    fn to_u8(self) -> u8 {
        match self {
            Self::Armed => 0,
            Self::Disarmed => 1,
            Self::AlarmTriggered => 2,
        }
    }

    fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Armed),
            1 => Some(Self::Disarmed),
            2 => Some(Self::AlarmTriggered),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Event {
    pub timestamp_ms: u64,
    pub kind: EventKind,
}

fn event_kind_key(idx: usize) -> Key {
    let mut buf = [0u8; 4];
    buf[0] = b'e';
    buf[1] = b'0' + (idx / 10) as u8;
    buf[2] = b'0' + (idx % 10) as u8;
    buf[3] = b'k';
    Key::from_slice(&buf)
}

fn event_ts_key(idx: usize) -> Key {
    let mut buf = [0u8; 4];
    buf[0] = b'e';
    buf[1] = b'0' + (idx / 10) as u8;
    buf[2] = b'0' + (idx % 10) as u8;
    buf[3] = b't';
    Key::from_slice(&buf)
}

pub struct EventLogger<'d> {
    nvs: Nvs<FlashStorage<'d>>,
    events: Vec<Event, MAX_EVENTS>,
}

impl<'d> EventLogger<'d> {
    pub fn new(flash: FlashStorage<'d>) -> Self {
        let nvs = Nvs::new(NVS_OFFSET, NVS_SIZE, flash).expect("NVS init failed");
        let mut logger = Self {
            nvs,
            events: Vec::new(),
        };
        logger.load();
        logger
    }

    fn load(&mut self) {
        let count: u8 = match self.nvs.get(&NS, &KEY_COUNT) {
            Ok(c) => c,
            Err(_) => return,
        };

        for i in 0..count as usize {
            let kind_key = event_kind_key(i);
            let ts_key = event_ts_key(i);

            let kind_u8: u8 = match self.nvs.get(&NS, &kind_key) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let ts: u64 = match self.nvs.get(&NS, &ts_key) {
                Ok(v) => v,
                Err(_) => continue,
            };

            if let Some(kind) = EventKind::from_u8(kind_u8) {
                let _ = self.events.push(Event {
                    timestamp_ms: ts,
                    kind,
                });
            }
        }
    }

    fn save(&mut self) {
        let count = self.events.len() as u8;
        if self.nvs.set(&NS, &KEY_COUNT, count).is_err() {
            warn!("NVS: failed to save count");
            return;
        }

        for (i, event) in self.events.iter().enumerate() {
            let kind_key = event_kind_key(i);
            let ts_key = event_ts_key(i);
            let _ = self.nvs.set(&NS, &kind_key, event.kind.to_u8());
            let _ = self.nvs.set(&NS, &ts_key, event.timestamp_ms);
        }
    }

    pub fn log(&mut self, kind: EventKind, timestamp_ms: u64) {
        if self.events.is_full() {
            let mut new: Vec<Event, MAX_EVENTS> = Vec::new();
            for e in self.events.iter().skip(1) {
                let _ = new.push(*e);
            }
            self.events = new;
        }
        let _ = self.events.push(Event { timestamp_ms, kind });
        self.save();
    }

    pub fn entries(&self) -> &[Event] {
        &self.events
    }

    pub fn count(&self) -> usize {
        self.events.len()
    }
}
