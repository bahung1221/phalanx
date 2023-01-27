
# UNDER CONSTRUCTION
This library is under construction, so use at your own risk

### Why Bulb?
Bulb is an lightweight microservices framework with simple, familiar syntax that was inspired by [ExpressJS](https://github.com/expressjs/express),
allow you to create transport layer for microservices in Rust as quick as possible.

Currently, Bulb is building around [nats](https://github.com/nats-io/nats-server) as central messaging system.
Bulb also have a local pubsub system, that allow services on same server communicate with each other fastest without remote messaging system.

### Concept:
![Concept](./assets/architecture.jpeg)
