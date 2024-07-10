use std::time::Duration;
use std::thread;
use roesti::tidal_service::TidalService;
use roesti::consumer1::Consumer1;
use roesti::consumer2::Consumer2;
use roesti::consumer3::Consumer3;

use dynamic_services_derive::dynamic_services_main;

#[dynamic_services_main]
fn main() {
    let ts = TidalService{
        location: "A".to_string()
    };
    let sreg = register_service(Box::new(ts));

    thread::sleep(Duration::from_secs(1));
    unregister_service(sreg);

    println!("Module path: {}", module_path!());
    println!("File: {}", file!());
    // TODO add a different service
}
