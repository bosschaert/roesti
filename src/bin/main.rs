use std::time::Duration;
use std::thread;
use roesti::tidal_service::TidalService;

use dynamic_services_derive::dynamic_services_main;

#[dynamic_services_main]
fn main() {
    let ts = TidalService{
        location: "A".to_string()
    };
    let sreg = register_service(Box::new(ts));

    thread::sleep(Duration::from_secs(1));
    unregister_service(sreg);

    // TODO add a different service
}
