use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use service::service::Service;
use transporters::base::Transporter;
use transporters::local::LocalTransporter;

mod transporters;
mod service;
mod event_bus;
mod context;

#[derive(Clone)]
pub struct Broker<T: Transporter> {
    remote_transporter: Rc<RefCell<T>>,
    local_transporter: Rc<RefCell<LocalTransporter>>,
    services: HashMap<String, Rc<RefCell<Service>>>,
}

pub trait BrokerTrait {
    fn create_service(&mut self, name: String) -> Rc<RefCell<Service>>;
}

impl<T: Transporter + 'static> Broker<T> {
    fn new(remote_transporter: T) -> Rc<RefCell<Self>> {
        let broker = Rc::new(RefCell::new(Broker { 
            remote_transporter: Rc::new(RefCell::new(remote_transporter)),
            local_transporter: Rc::new(RefCell::new(LocalTransporter::new())),
            services: HashMap::new(),
        }));

        Broker::subscribe_local_transporter(broker.clone());
        broker
    }

    fn subscribe_local_transporter(broker: Rc<RefCell<Broker<T>>>) {
        let broker_cloned = broker.clone();
        let binding = &broker.clone();
        let local_transporter = &binding.borrow().local_transporter;
    
        local_transporter
            .borrow_mut()
            .subscribe("call".to_owned(), Box::new(move |ctx| {
                let ctx_cloned = ctx.clone();
                let target = ctx.req.service;
                let action = ctx.req.action;
    
                println!("target: {}, action: {}", target, action);
    
                let mut action_listener_name: String = target.clone();
                action_listener_name.push_str(&":".clone());
                action_listener_name.push_str(&action.clone());
    
                let services = &broker_cloned.borrow().services;
                let target_exists_in_local = services.contains_key(&target);
                if target_exists_in_local {
                    // Just send the message to that service locally
                    broker_cloned.borrow().local_transporter.borrow().publish(
                        action_listener_name,
                        ctx_cloned,
                    );
                    return;
                } else {
                    // Target service doesn't exists within the same broker
                    // So send the message to the remote transporter instead
                    broker_cloned.borrow().remote_transporter.borrow().publish(
                        action_listener_name,
                        ctx_cloned,
                    );
                }
            }));
    }
}


impl<T: Transporter> BrokerTrait for Broker<T> {
    fn create_service(&mut self, name: String) -> Rc<RefCell<Service>> {
        let service = Rc::new(RefCell::new(
            Service {
                name: name.clone(),
                local_transporter: self.local_transporter.clone(),
            }
        ));

        self.services.insert(name.clone(), service.clone());

        service.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use crate::{transporters::nats::NatsTransporter, context::context::Context};

    use super::*;

    #[test]
    fn should_able_to_create_broker_and_service() {
        let broker = Broker::new(NatsTransporter::new());
        let broker_cloned = broker.clone();
        let broker_transporter = &broker_cloned
            .borrow()
            .remote_transporter
            .clone();

        assert!(
            print_type_of(broker_transporter).contains("NatsTransporter")
        );

        let foo_service = broker.borrow_mut().create_service("foo".to_owned());
        let bar_service = broker.borrow_mut().create_service("bar".to_owned());

        assert_eq!(foo_service.borrow().name, "foo".to_owned());
        assert_eq!(bar_service.borrow().name, "bar".to_owned());
    }

    #[test]
    fn local_transporter() {
        let broker = Broker::new(NatsTransporter::new());

        let foo_service = broker.borrow_mut().create_service("foo".to_owned());
        let bar_service = broker.borrow_mut().create_service("bar".to_owned());

        foo_service.borrow_mut().subscribe("hello".to_owned(), Box::new(|ctx| {
            println!("Foo Hello World!: {:?}", ctx);
        }));

        thread::sleep(Duration::from_millis(4000));

        bar_service.borrow().call(
            Context::default(),
            String::from("foo"), 
            String::from("hello"), 
            String::from("sample data"),
        );
    }

    fn print_type_of<T>(_: &T) -> String {
        std::any::type_name::<T>().to_owned()
    }
}
