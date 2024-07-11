use std::collections::BTreeMap;
use std::time::Duration;
use std::thread;
use roesti::tidal_service::TidalService;

use dynamic_services_derive::dynamic_services_main;

#[dynamic_services_main]
fn main() {
    let ts = TidalService{
        location: "A".to_string()
    };

    let mut props = BTreeMap::new();
    props.insert("foo".to_owned(), "bar".to_owned());
    props.insert("hello".to_owned(), "123".to_owned());

    let sreg = register_service(Box::new(ts), props);

    thread::sleep(Duration::from_secs(1));
    unregister_service(sreg);

    // TODO add a different service
}
