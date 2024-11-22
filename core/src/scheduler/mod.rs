use event::{Event, EventType, FutureEvent};
use std::collections::BinaryHeap;

pub mod event;

pub struct Scheduler {
    timestamp: usize,
    events: BinaryHeap<Event>,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            timestamp: 0,
            events: BinaryHeap::new(),
        }
    }

    pub fn peek(&self) -> Option<EventType> {
        self.events.peek().map(|e| e.event_type())
    }

    pub fn pop(&mut self) -> Option<(EventType, usize)> {
        match self.events.peek() {
            Some(event) => {
                if self.timestamp >= event.time() {
                    let event = self.events.pop().unwrap_or_else(|| unreachable!());
                    Some((event.event_type(), event.time()))
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn cancel_events(&mut self, event_type: EventType) {
        let mut new_events = BinaryHeap::new();
        self.events
            .iter()
            .filter(|e| e.event_type() != event_type)
            .for_each(|e| new_events.push(e.clone()));
        self.events = new_events
    }

    pub fn schedule(&mut self, event: FutureEvent) {
        let (event_type, delta_time) = event;
        let event = Event::new(event_type, self.timestamp + delta_time);
        self.events.push(event);
    }

    pub fn schedule_at(&mut self, event_type: EventType, timestamp: usize) {
        self.events.push(Event::new(event_type, timestamp));
    }

    pub fn cycles_until_next_event(&self) -> usize {
        if let Some(event) = self.events.peek() {
            event.time() - self.timestamp
        } else {
            0
        }
    }

    pub fn update(&mut self, cycles: usize) {
        self.timestamp += cycles;
    }

    pub fn update_to_next_event(&mut self) {
        self.timestamp += self.cycles_until_next_event();
    }

    pub fn timestamp_of_next_event(&self) -> usize {
        if let Some(event) = self.events.peek() {
            event.time()
        } else {
            //0
            panic!("No events")
        }
    }

    pub fn timestamp(&self) -> usize {
        self.timestamp
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}
