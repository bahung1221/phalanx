# Phalanx

Microservices framework for Rust.

## UNDER CONSTRUCTION
This library is under construction, only use it if you want to experiment.
Any contribution is welcome.

### Why Phalanx?
**Phalanx** is an lightweight microservices framework with simple, familiar syntax that was inspired by [ExpressJS](https://github.com/expressjs/express),
allow you to create transport layer for microservices in Rust as quick as possible.

Currently, **Phalanx** is building around [nats](https://github.com/nats-io/nats-server) as central messaging system.
**Phalanx** also have a local pubsub system, that allow services on same server communicate with each other fastest without remote messaging system.

### Concept:
![Concept](./assets/architecture.jpeg)

### Usages

**NOTE**: This is just the ideal flow of Phalanx, the framework is still under construction.

```rs
let transporter = NatsTransporter::new("nats://demo.nats.io:4222".into()).await;
let mut broker = Broker::new(transporter).await;

let foo_service = broker.create_service("foo".to_owned()).await;
let bar_service = broker.create_service("bar".to_owned()).await;

// "Foo" is a subscriber service
foo_service
    .lock()
    .await
    .subscribe("hello".to_owned(), Box::new(|ctx| {
        println!("Foo Hello World!: {:?}", ctx);
    }))
    .await;

// "Bar" call "foo" by publish an action
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
```

### Roadmap

- [ ] Request-Reply
- [ ] Transporter
    - [ ] Nats
    - [ ] TCP
    - [ ] Kafka
- [ ] Serializers
    - [ ] JSON
    - [ ] Protocol Buffer
- [ ] API Gateway
- [ ] Service Registry
    - [ ] Service discovery
    - [ ] Heath check
- [ ] Fault tolerance
    - [ ] Load balancer (built-in if using Nats or Kafka).
    - [ ] Circuit breaker.
    - [ ] Retries & Timeout.
    - [ ] Bulkhead.
- [ ] Middleware
- [ ] Logger
- [ ] Metrics/Services monitoring cli tool
- [ ] Stream (for streaming files through services,...)

