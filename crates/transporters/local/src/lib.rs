use std::io;
use phalanx_transporter_core::Subscriber;
use phalanx_transporter_core::Transporter;
use phalanx_transporter_core::context::Context;
use event_bus::event_bus::EventBus;

mod event_bus;

pub struct LocalTransporter {
    event_bus: EventBus,
}

impl LocalTransporter {
    pub fn new() -> LocalTransporter {
        LocalTransporter {
            event_bus: EventBus::default(),
        }
    }
}

impl Transporter for LocalTransporter {
    fn connect(&mut self) -> Result<bool, io::Error> {
        return Ok(true);
    }

    fn disconnect(&mut self) -> Result<bool, io::Error> {
        return Ok(true);
    }

    fn subscribe(&mut self, subject: String, listener: Box<Subscriber>) {
        self.event_bus.subscribe(subject.clone(), listener);
    }

    fn publish(&self, subject: String, data: Context) {
        self.event_bus.publish(subject, data);
    }
}