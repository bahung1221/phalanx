use std::io;
use async_trait::async_trait;
use futures::future;
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

#[async_trait]
impl Transporter for LocalTransporter {
    async fn subscribe(&mut self, subject: String, listener: Box<(dyn Fn(Context) + 'static + Send + Sync)>) {
        self.event_bus.subscribe(subject.clone(), listener);

        future::ready(true);
    }

    async fn publish(&self, subject: String, data: Context) {
        self.event_bus.publish(subject, data);

        future::ready(true);
    }
}