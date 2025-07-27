use crate::Event;

pub struct EventQueue<E: Event> {
    events: Vec<E>,
}

impl<E: Event> EventQueue<E> {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn push_event(&mut self, event: E) {
        self.events.push(event);
    }

    pub fn drain(&mut self) -> Vec<E> {
        self.events.drain(..).collect()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

impl<E: Event> Default for EventQueue<E> {
    fn default() -> Self {
        Self::new()
    }
}
