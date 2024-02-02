use std::any::Any;
use roesti::{service_registry::ServiceRegistry, tidal_service::TidalService};

fn main() {
    let mut sr = ServiceRegistry::new();

    let ts = TidalService{
        location: "A".to_string()
    };
    sr.register_service("TidalService", Box::new(ts));

    let ts2 = TidalService{
        location: "B".to_string()
    };
    sr.register_service("TidalService", Box::new(ts2));


    let tsx = sr.get_service("TidalService");
    match tsx {
        Some(s) => use_service(s),
        _ => ()
    }

    let services = sr.get_services_by_name("TidalService");
    for s in services {
        use_service(s);
    }

    let tsy = sr.get_svc::<TidalService>("TidalService");
    match tsy {
        Some(s) => call_tidal(s),
        _ => ()
    }

    let svcs = sr.get_svcs::<TidalService>("TidalService");
    for s in svcs {
        call_tidal(s);
    }
}

fn call_tidal(ts: &TidalService) {
    println!("YoYo: {}", ts.location());
}

fn use_service(ts: &dyn Any) {
    println!("Got service {:?}", ts);

    let x = ts.downcast_ref::<TidalService>().unwrap();
    println!("{}: Next event {}", x.location(), x.next_event());
}