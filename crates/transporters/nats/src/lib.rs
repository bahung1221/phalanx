use async_nats::Subscriber;
use async_trait::async_trait;
use futures::StreamExt;
use futures::future;
// use phalanx_transporter_core::Subscriber;
use phalanx_transporter_core::Transporter;
use phalanx_transporter_core::context::Context;
use phalanx_transporter_core::context::IncomingRequest;

#[derive(Clone, Debug)]
pub struct NatsTransporter {
    server_addr: String,
    client: async_nats::Client,
}

impl NatsTransporter {
    pub async fn new(server_addr: String) -> NatsTransporter {
        let client = async_nats::connect(server_addr.clone()).await.unwrap();

        NatsTransporter {
            server_addr: server_addr.clone(),
            client,
        }
    }
}

#[async_trait]
impl Transporter for NatsTransporter {
    async fn subscribe(&mut self, subject: String, listener: Box<(dyn Fn(Context) + 'static + Send + Sync)>) {
        let mut subscriber = self.client.subscribe(subject.clone()).await.unwrap();

        while let Some(msg) = subscriber.next().await {
            println!("{:?}", msg);

            let payload = msg.payload.to_vec();
            let payload = std::str::from_utf8(&payload);
            let mut context = Context::default();
            context.req = IncomingRequest {
                service: "sample service".into(),
                action: "sample action".into(),
                body: payload.unwrap().into(),
            };
            listener(context);
        }
    }

    async fn publish(&self, subject: String, data: Context) {
        let bytes = bytes::Bytes::from(data.to_json());

        self.client.publish(subject, bytes).await.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use futures::future::abortable;

    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn should_work() {
        let transporter = NatsTransporter::new("nats://demo.nats.io:4222".into()).await;
        let mut transporter_cloned = transporter.clone();
        let transporter_cloned_2 = transporter.clone();

        let rt = tokio::runtime::Handle::try_current().unwrap();

        let (t1, t1_handle) = abortable(async move {
            println!("Subscribed!");

            transporter_cloned.subscribe("phalanx:test-channel".into(), Box::new(|ctx| {
                println!("Hello World!: {:?}", ctx);
            })).await;
        });
    
        let t2 = rt.spawn(async move {
            for index in 0..10 {
                transporter_cloned_2.publish("phalanx:test-channel".into(), Context::default()).await;
                println!("Transported: {:?}", index);
            }
        });

        let t3 = rt.spawn(async move {
            thread::sleep(Duration::from_millis(4000));

            println!("Aborting the listener!");
            t1_handle.abort();
        });

        let (r1, r2, r3) = tokio::join!(
            rt.spawn(t1), 
            t2,
            t3,
        );

        let r1 = r1.unwrap();
        r2.unwrap();
        r3.unwrap();

        match r1 {
            Ok(_) => todo!(),
            Err(_) => assert!(true),
        }
    }
}