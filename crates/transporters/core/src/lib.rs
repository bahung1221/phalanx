use async_trait::async_trait;
use context::Context;

pub mod context;

/// A subscriber (listener) has type of a callable function.
pub type Subscriber = dyn Fn(Context) + Send + Sync;

#[async_trait]
pub trait Transporter {
    async fn subscribe(&mut self, subject: String, listener: Box<Subscriber>);
    async fn publish(&self, subject: String, data: Context);
}