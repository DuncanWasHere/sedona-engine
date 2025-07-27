use crate::Event;
use std::collections::HashMap;

pub struct EventHandlers<E: Event, W, R> {
    pub handlers: HashMap<E::Key, Vec<fn(&E, &mut W, &mut R)>>,
}

impl<E: Event, W, R> EventHandlers<E, W, R> {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register(&mut self, key: E::Key, handler: fn(&E, &mut W, &mut R)) {
        self.handlers.entry(key).or_default().push(handler);
    }

    pub fn dispatch(&self, event: E, world: &mut W, resources: &mut R) {
        let key = event.key();

        if let Some(handlers) = self.handlers.get(&key) {
            for handler in handlers {
                handler(&event, world, resources);
            }
        }
    }
}

impl<E: Event, W, R> Default for EventHandlers<E, W, R> {
    fn default() -> Self {
        Self::new()
    }
}
