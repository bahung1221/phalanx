use std::{cell::RefCell, rc::Rc};

use crate::{
    context::context::{Context, IncomingRequest},
    event_bus::event_bus::{Subscriber}, 
    transporters::{local::LocalTransporter, base::Transporter},
};

#[derive(Clone)]
pub struct Service {
    pub name: String,
    pub local_transporter: Rc<RefCell<LocalTransporter>>,
}

impl Service {
    fn new(name: String, local_transporter: Rc<RefCell<LocalTransporter>>) -> Self {
        Service {
            name,
            local_transporter,
        }
    }

    pub fn subscribe(&mut self, action: String, listener: Box<Subscriber>) {
        let mut action_listener_name: String = self.name.clone();
        action_listener_name.push_str(&":".clone());
        action_listener_name.push_str(&action.clone());

        self.local_transporter
            .borrow_mut()
            .subscribe(action_listener_name, listener);
    }

    pub fn call(&self, ctx: Context, target_service: String, action: String, data: String) {
        let mut new_ctx = ctx.clone();

        new_ctx.metadata.request_chains.push(self.name.clone());
        new_ctx.req = IncomingRequest {
            service: target_service.clone(),
            action: action.clone(),
            body: data.clone(),
        };

        self.local_transporter.borrow().publish("call".to_owned(), new_ctx);
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn it_works() {
        let local_transporter = Rc::new(
            RefCell::new(
                LocalTransporter::new()
            )
        );

        let mut service1 = Service {
            name: "service-1".to_owned(),
            local_transporter: local_transporter.clone(),
        };
        let service2 = Service {
            name: "service-2".to_owned(),
            local_transporter: local_transporter.clone(),
        };

        service1.subscribe(String::from("hello"), Box::new(|ctx| {
            println!(
                "\"{}\" has been called with data: {:?}",
                String::from("service-1:hello"),
                ctx,
            );
        }));

        thread::sleep(Duration::from_millis(4000));

        service2.call(
            Context::default(),
            String::from("service-1"),
            String::from("hello"),
            String::from("sample data"),
        );
    }
}
