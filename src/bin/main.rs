use once_cell::sync::Lazy;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::thread;
use roesti::tidal_service::TidalService;
use roesti::consumer1::Consumer1;
use roesti::consumer2::Consumer2;
use roesti::service_registry::{ConsumerRegistration, ServiceReference, ServiceRegistration, REGD_SERVICES};

use dynamic_services_derive::dynamic_services_main;

// Probably not needed as a macro_rules
macro_rules! register_service {
    ($svc:expr) => {
        register_service($svc)
    };
}

#[dynamic_services_main]
fn main() {
    let ts = TidalService{
        location: "A".to_string()
    };
    let sreg = register_service!(Box::new(ts));

    thread::sleep(Duration::from_secs(1));
    unregister_service(sreg);

    // thread::sleep(Duration::MAX);

    // TODO add a different service
}
