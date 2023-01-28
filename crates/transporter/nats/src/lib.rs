use std::io;
use bulb_transporter_core::Subscriber;
use bulb_transporter_core::Transporter;
use bulb_transporter_core::context::Context;

pub struct NatsTransporter;

impl NatsTransporter {
    pub fn new() -> NatsTransporter {
        NatsTransporter {}
    }
}

impl Transporter for NatsTransporter {
    fn connect(&mut self) -> Result<bool, io::Error> {
        return Ok(true);
    }

    fn disconnect(&mut self) -> Result<bool, io::Error> {
        return Ok(true);
    }

    fn subscribe(&mut self, subject: String, listener: Box<Subscriber>) {
        todo!()
    }

    fn publish(&self, subject: String, data: Context) {
        todo!()
    }
}
