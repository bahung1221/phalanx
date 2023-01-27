use std::{collections::HashMap};

use crate::context::context::Context;

/// A subscriber (listener) has type of a callable function.
pub type Subscriber = dyn Fn(Context);

#[derive(Default)]
pub struct EventBus {
    events: HashMap<String, Vec<Box<Subscriber>>>,
}

impl EventBus {
    pub fn subscribe(&mut self, event_type: String, listener: Box<Subscriber>) {
        self.events.entry(event_type.clone()).or_default();
        self.events.get_mut(&event_type).unwrap().push(listener);
    }

    pub fn unsubscribe(&mut self, event_type: String, listener: Box<Subscriber>) {
        self.events
            .get_mut(&event_type)
            .unwrap()
            .retain(|&x| x != listener);
    }

    pub fn notify(&self, event_type: String, data: Context) {
        let listeners = self.events.get(&event_type).unwrap();
        for listener in listeners {
            listener(data.clone());
        }
    }

    pub fn publish(&self, event_type: String, data: Context) {
        self.notify(event_type, data.clone());
    }
}