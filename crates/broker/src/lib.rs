use std::collections::HashMap;
use std::sync::{Arc};
use async_trait::async_trait;
use futures::lock::Mutex;

// use phalanx_transporter_core::context::Context;
use service::service::Service;
use phalanx_transporter_core::Transporter;
// use phalanx_transporter_local::LocalTransporter;

mod service;

#[derive(Clone)]
pub struct Broker<T: Transporter> {
    transporter: Arc<Mutex<T>>,
    services: Arc<Mutex<HashMap<String, Arc<Mutex<Service<T>>>>>>,
}

#[async_trait]
pub trait BrokerTrait<T: Transporter> {
    async fn create_service(&mut self, name: String) -> Arc<Mutex<Service<T>>>;
}

// async fn onBrokerCall<T: Transporter>Service(name.clone(), transporter)  let local_transporter = broker.local_transporter.clone();
//     let remote_transporter = broker.remote_transporter.clone();
//     let services = broker.services.clone();

//     let ctx_cloned = ctx.clone();
//     let target = ctx.req.service;
//     let action = ctx.req.action;

//     println!("target: {}, action: {}", target, action);

//     let mut action_listener_name: String = target.clone();
//     action_listener_name.push_str(&":".clone());
//     action_listener_name.push_str(&action.clone());

//     let rt = tokio::runtime::Handle::try_current().unwrap();
//     let target_exists_in_local = services.lock().await.contains_key(&target);

//     println!("1: {}", action_listener_name);
//     if target_exists_in_local {
//         // Just send the message to that service locally
//         println!("a");
//         let local_transporter = local_transporter.lock().await;
//         println!("b");
//         local_transporter.publish(
//             action_listener_name,
//             ctx_cloned,
//         ).await;
//         println!("c");
//         return;
//     }

//     println!("d");

//     // Target service doesn't exists within the same broker
//     // So send the message to the remote transporter instead
//     let remote_transporter = remote_transporter.lock().await;
//     remote_transporter.publish(
//         action_listener_name,
//         ctx_cloned,
//     ).await;
// }

impl<T: Transporter + Send + 'static> Broker<T> {
    async fn new(transporter: T) -> Self {
        let broker = Broker { 
            transporter: Arc::new(Mutex::new(transporter)),
            services: Arc::new(Mutex::new(HashMap::new())),
        };

        broker
    }
}

#[async_trait]
impl<T: Transporter + Send> BrokerTrait<T> for Broker<T> {
    async fn create_service(&mut self, name: String) -> Arc<Mutex<Service<T>>> {
        let transporter = self.transporter.clone();

        let service = Arc::new(
            Mutex::new(
                Service {
                    name: name.clone(),
                    transporter,
                }
            ),
        );

        self.services.lock().await.insert(name.clone(), service.clone());

        service
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use phalanx_transporter_core::context::Context;
    use phalanx_transporter_local::LocalTransporter;
    use phalanx_transporter_nats::NatsTransporter;

    use super::*;

    #[tokio::test()]
    async fn should_able_to_create_broker_and_service() {
        println!("here");
        let transporter = LocalTransporter::new();
        let mut broker = Broker::new(transporter).await;
        println!("here2");
        let broker_transporter = &broker.transporter;

        assert!(
            print_type_of(broker_transporter).contains("LocalTransporter")
        );

        let foo_service = broker.create_service("foo".to_owned()).await;
        let bar_service = broker.create_service("bar".to_owned()).await;

        assert_eq!(foo_service.lock().await.name, "foo".to_owned());
        assert_eq!(bar_service.lock().await.name, "bar".to_owned());
    }

    #[tokio::test()]
    async fn nats_transporter() {
        let transporter = NatsTransporter::new("nats://demo.nats.io:4222".into()).await;
        let mut broker = Broker::new(transporter).await;

        let foo_service = broker.create_service("foo".to_owned()).await;
        let bar_service = broker.create_service("bar".to_owned()).await;

        foo_service
            .lock()
            .await
            .subscribe("hello".to_owned(), Box::new(|ctx| {
                println!("Foo Hello World!: {:?}", ctx);
            }))
            .await;

        thread::sleep(Duration::from_millis(100));

        println!("Calling foo service");
    
        bar_service
            .lock()
            .await
            .call(
                Context::default(),
                String::from("foo"),
                String::from("hello"),
                String::from("sample data"),
            )
            .await;
    }

    fn print_type_of<T>(_: &T) -> String {
        std::any::type_name::<T>().to_owned()
    }
}
