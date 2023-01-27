use std::io;

use crate::{event_bus::event_bus::Subscriber, context::context::Context};

pub trait Transporter {
    fn connect(&mut self) -> Result<bool, io::Error>;
    fn disconnect(&mut self) -> Result<bool, io::Error>;
    fn subscribe(&mut self, subject: String, listener: Box<Subscriber>);
    fn publish(&mut self, subject: String, data: Context);
}