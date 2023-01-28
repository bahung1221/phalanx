use std::collections::HashMap;

use bulb_transporter_core::Subscriber;
use bulb_transporter_core::context::Context;

#[derive(Default)]
pub struct EventBus {
    events: HashMap<String, Box<Subscriber>>,
}

impl EventBus {
    pub fn subscribe(&mut self, event_type: String, listener: Box<Subscriber>) {
        self.events.insert(event_type.clone(), listener);
    }

    pub fn unsubscribe(&mut self, event_type: String) {
        self.events.remove(&event_type.clone());
    }

    pub fn notify(&self, event_type: String, data: Context) {
        let listener = self.events.get(&event_type);
        if listener.is_none() {
            return;
        }

        listener.unwrap()(data.clone());
    }

    pub fn publish(&self, event_type: String, data: Context) {
        self.notify(event_type, data.clone());
    }
}
