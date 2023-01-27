use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use service::service::Service;
use transporters::base::Transporter;
use event_bus::event_bus::EventBus;

mod transporters;
mod service;
mod event_bus;
mod context;

#[derive(Clone)]
pub struct Broker<T: Transporter> {
    transporter: T,
    broker_event_bus: Rc<RefCell<EventBus>>,
    services: HashMap<String, Rc<RefCell<Service>>>,
}

pub trait BrokerTrait {
    fn create_service(&mut self, name: String) -> Rc<RefCell<Service>>;
}

impl<T: Transporter + 'static> Broker<T> {
    fn new(transporter: T) -> Rc<RefCell<Self>> {
        let broker = Rc::new(RefCell::new(Broker { 
            transporter,
            broker_event_bus: Rc::new(RefCell::new(EventBus::default())),
            services: HashMap::new(),
        }));

        {
            let broker_cloned_1 = broker.clone();
            let broker_cloned_2 = &broker.clone();
            let broker_event_bus = &broker_cloned_2.borrow().broker_event_bus;
    
            broker_event_bus
                .borrow_mut()
                .subscribe("call".to_owned(), Box::new(move |ctx| {
                    let target = ctx.req.service;
                    let action = ctx.req.action;
    
                    println!("target: {}, action: {}", target, action);
    
                    if broker_cloned_1.borrow().services.contains_key(&target) {
                        println!("Founded!");
                    }
                }));
        }

        broker
    }
}

impl<T: Transporter> BrokerTrait for Broker<T> {
    fn create_service(&mut self, name: String) -> Rc<RefCell<Service>> {
        let service = Rc::new(RefCell::new(
            Service {
                name: name.clone(),
                broker_event_bus: self.broker_event_bus.clone(),
            }
        ));

        self.services.insert(name.clone(), service.clone());

        service.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use crate::{transporters::local::LocalTransporter, context::context::Context};

    use super::*;

    #[test]
    fn local_transporter() {
        let broker = Broker::new(LocalTransporter::new());

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

    #[test]
    fn loadtest() {
        let broker = Broker::new(LocalTransporter::new());

        let foo_service = broker.borrow_mut().create_service("foo".to_owned());
        let bar_service = broker.borrow_mut().create_service("bar".to_owned());

        foo_service.borrow_mut().subscribe("hello".to_owned(), Box::new(|ctx| {
            println!("Foo Hello World!: {:?}", ctx);
        }));

        // thread::sleep(Duration::from_millis(4000));

        let number = 10000; 
        for num in 0..number { // change it to get range
            println!("Number: {}", num);
            bar_service.borrow().call(
                Context::default(),
                String::from("foo"), 
                String::from("hello"), 
                String::from("sample data"),
            );
        }

    }
}
