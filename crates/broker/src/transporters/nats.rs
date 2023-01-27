use std::io;

use super::base;
use crate::{event_bus::event_bus::Subscriber, context::context::Context};

struct NatsTransporter;
impl base::Transporter for NatsTransporter {
    fn connect(&mut self) -> Result<bool, io::Error> {
        return Ok(true);
    }

    fn disconnect(&mut self) -> Result<bool, io::Error> {
        return Ok(true);
    }

    fn subscribe(&mut self, subject: String, listener: Box<Subscriber>) {
        println!("Subscribed for: {}", subject);
        todo!()
    }

    fn publish(&mut self, subject: String, data: Context) {
        todo!()
    }
}