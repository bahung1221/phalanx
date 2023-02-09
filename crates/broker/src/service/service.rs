use std::sync::{Arc};
use futures::lock::Mutex;

use phalanx_transporter_core::{Transporter, Subscriber};
use phalanx_transporter_core::context::{Context, IncomingRequest};

#[derive(Clone)]
pub struct Service<T: Transporter> {
    pub name: String,
    pub transporter: Arc<Mutex<T>>,
}

impl<T: Transporter> Service<T> {
    fn new(
        name: String, 
        transporter: Arc<Mutex<T>>,
    ) -> Self {
        Service {
            name,
            transporter,
        }
    }

    pub async fn subscribe(&mut self, action: String, listener: Box<Subscriber>) {
        let mut action_listener_name = self.name.clone();
        action_listener_name.push_str(&":".clone());
        action_listener_name.push_str(&action.clone());

        println!("2: {}", action_listener_name);

        self.transporter
            .lock()
            .await
            .subscribe(action_listener_name, listener)
            .await;
    }

    pub async fn call(&self, ctx: Context, target_service: String, action: String, data: String) {
        let mut new_ctx = ctx.clone();

        new_ctx.metadata.request_chains.push(self.name.clone());
        new_ctx.req = IncomingRequest {
            service: target_service.clone(),
            action: action.clone(),
            body: data.clone(),
        };

        let mut target = target_service.clone();
        target.push_str(&":".clone());
        target.push_str(&action.clone());

        println!("3: {}", target);

        self.transporter
            .lock()
            .await
            .publish(target.into(), new_ctx)
            .await;
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use phalanx_transporter_local::LocalTransporter;

    use super::*;

    #[tokio::test()]
    async fn it_works() {
        let transporter = Arc::new(
            Mutex::new(
                LocalTransporter::new()
            ),
        );

        let mut service1 = Service {
            name: "service-1".to_owned(),
            transporter: transporter.clone(),
        };
        let service2 = Service {
            name: "service-2".to_owned(),
            transporter: transporter.clone(),
        };

        service1.subscribe(String::from("hello"), Box::new(|ctx| {
            println!(
                "\"{}\" has been called with data: {:?}",
                String::from("service-1:hello"),
                ctx,
            );
        })).await;

        thread::sleep(Duration::from_millis(4000));

        service2.call(
            Context::default(),
            String::from("service-1"),
            String::from("hello"),
            String::from("sample data"),
        ).await;
    }
}
