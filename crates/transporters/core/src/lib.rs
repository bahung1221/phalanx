use std::io;

use context::Context;

pub mod context;

/// A subscriber (listener) has type of a callable function.
pub type Subscriber = dyn Fn(Context);

pub trait Transporter {
    fn connect(&mut self) -> Result<bool, io::Error>;
    fn disconnect(&mut self) -> Result<bool, io::Error>;
    fn subscribe(&mut self, subject: String, listener: Box<Subscriber>);
    fn publish(&self, subject: String, data: Context);
}