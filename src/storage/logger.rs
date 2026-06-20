use heapless::Vec;

#[derive(Debug, Clone, Copy, PartialEq, defmt::Format)]
pub enum EventKind {
    Armed,
    Disarmed,
    AlarmTriggered,
}

#[derive(Debug, Clone, Copy)]
pub struct Event {
    pub timestamp_ms: u64,
    pub kind: EventKind,
}

pub struct EventLogger {
    events: Vec<Event, 32>,
}

impl EventLogger {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn log(&mut self, kind: EventKind, timestamp_ms: u64) {
        if self.events.is_full() {
            let mut new: Vec<Event, 32> = Vec::new();
            for e in self.events.iter().skip(1) {
                let _ = new.push(*e);
            }
            self.events = new;
        }
        let _ = self.events.push(Event { timestamp_ms, kind });
    }

    pub fn entries(&self) -> &[Event] {
        &self.events
    }

    pub fn count(&self) -> usize {
        self.events.len()
    }
}

impl Default for EventLogger {
    fn default() -> Self {
        Self::new()
    }
}
