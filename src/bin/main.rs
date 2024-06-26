use std::any::Any;
use std::time::Duration;
use std::thread;
use dynamic_services_derive::dynamic_services;
use roesti::{test_service_registry::TestServiceRegistry, sunlight_service::SunlightService, tidal_service::TidalService, location::Location};
use roesti::consumer1::Consumer1;
use roesti::service_registry::ServiceRegistry;

// Note that this is the last annotation to get processed, so it could process all the annotations previously from a file
// #[dynamic_services]
fn main() {
    let mut sr = ServiceRegistry::default();
    // sr.register_service(TidalService{
    //     location: "A".to_string()
    // });

    let mut svcs = vec![];
    let ts = TidalService{
        location: "A".to_string()
    };
    svcs.push(ts);

    let tsref = svcs.get(0).unwrap();

    let mut c1 = Consumer1::default();
    c1.set_TidalService(tsref);
    println!("c1: {}", c1);

    // Consumer2::new();
    /*
    let x = 12;
    let y = 42;
    dp!(x, y);
     */

    thread::sleep(Duration::MAX);

    mainx();
}

fn mainx() {
    if 1 == 1 {
        return; // Ignore the rest for now
    }

    let mut sr = TestServiceRegistry::new();

    let ts = TidalService{
        location: "A".to_string()
    };
    sr.register_service("TidalService", Box::new(ts));

    let ts2 = TidalService{
        location: "B".to_string()
    };
    sr.register_service("TidalService", Box::new(ts2));


    let sls = SunlightService{
        location: "C".to_string()
    };
    sr.register_service("SunlightService", Box::new(sls));

    let tsx = sr.get_service_by_name("TidalService");
    match tsx {
        Some(s) => use_service(s),
        _ => ()
    }

    let services = sr.get_services_by_name("TidalService");
    for s in services {
        use_service(s);
    }

    let tsy = sr.get_svc::<TidalService>();
    match tsy {
        Some(s) => call_tidal(s),
        _ => ()
    }

    println!("Calling first service found");
    if let Some(tsz) = sr.get_svc::<TidalService>() {
        println!("loc: {}", tsz.location());
    }

    let svcs = sr.get_svcs::<TidalService>();
    for s in svcs {
        println!("List loc: {}", s.location());
    }

    // let svcs = sr.get_service_by_name("Location");
    // for s in svcs {
    //     println!("List loc: {}", s.location());
    // }

    let all_svcs = sr.get_all_svcs();
    for s in all_svcs {
        println!("svc: {:?} - typeid {:?}", s, s.type_id());

        // let r = s.as_ref();
        // let x = r.downcast_ref::<dyn Location>();
        if s.is::<Box<dyn Location>>() {
            println!("location!");
        }
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