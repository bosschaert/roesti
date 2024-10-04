use std::boxed::Box;
use std::collections::BTreeMap;
use roesti::sunlight_service::SunlightService;
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

    let sreg = register_service(Box::new(ts), props.clone());

    let sls = SunlightService{
        location: "B".to_string()
    };
    let _sreg2 = register_service(Box::new(sls), BTreeMap::new());

    props.remove("hello");
    props.insert("hi".to_string(), "ha".to_string());
    update_service(&sreg, props);

    unregister_service(sreg);
}
