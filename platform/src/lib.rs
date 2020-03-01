/// Core Module
mod core {

    use std::collections::VecDeque;

    pub struct DynamicRoutingSlip {
        routes: VecDeque<Route>,
        in_progress: bool
    }

    impl DynamicRoutingSlip {

        pub fn start(&self) {

        }
        pub fn next_route(&mut self) {

        }
        pub fn peek_at_next_route(&self) {

        }
        pub fn number_remaining_routes(&self) {

        }
    }

    pub struct Envelope {
        pub id: i64,
        pub client: i64,
        pub reply_to_client: bool,
        pub client_reply_action: String,
        pub external: bool,
        pub routing_slip: DynamicRoutingSlip,
        pub man_con: ManCon,
        pub min_delay: i64,
        pub max_delay: i64
    }

    pub enum ManCon {
        NEO, EXTREME, VERYHIGH, HIGH, MEDIUM, LOW, NONE, UNKNOWN
    }

    pub struct Route {
        pub id: i64,
        pub service: String,
        pub op: String,
        pub routed: bool

    }

    pub trait Service {
        fn handle(&self);
    }

    struct Context {

    }

    impl Context {

    }

    struct ServiceBus {

    }

    impl ServiceBus {

    }

    struct Orchestration {

    }

    impl Orchestration {

    }

    pub struct Client {

    }

    impl Client {

    }

}

mod services {

    struct Admin {

    }

    struct Identity {

    }

    struct InfoVault {

    }

    struct KeyRing {

    }

    struct Monetary {

    }

    struct Network {

    }

    struct Notification {

    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

