use once_cell::sync::Lazy;
use std::any::Any;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::thread;
use roesti::{test_service_registry::TestServiceRegistry, sunlight_service::SunlightService, tidal_service::TidalService, location::Location};
use roesti::consumer1::Consumer1;
use roesti::consumer2::Consumer2;
use roesti::service_registry::ServiceRegistry;

use dynamic_services_derive::dynamic_services_main;

// static CONSUMERS_INITIALIZED: Lazy<Mutex<bool>> = Lazy::new(||Mutex::new(false));
static CONSUMERS_INITIALIZED: AtomicBool = AtomicBool::new(false);
// static SERVICES: Lazy<Mutex<Vec<Box<dyn Any + Send + Sync>>>> = Lazy::new(||Mutex::new(Vec::new()));
// static TIDAL_SERVICES: Lazy<Mutex<Vec<TidalService>>> = Lazy::new(||Mutex::new(Vec::new()));
static TIDAL_CONSUMER1: Lazy<Mutex<Vec<fn() -> Consumer1<'static>>>> = Lazy::new(||Mutex::new(Vec::new()));
static TIDAL_CONSUMER2: Lazy<Mutex<Vec<fn() -> Consumer2<'static>>>> = Lazy::new(||Mutex::new(Vec::new()));

macro_rules! register_service {
    ($svc:expr) => {
        register_consumers();
        register_service($svc);
    };
}

#[dynamic_services_main]
fn main() {
    let ts = TidalService{
        location: "A".to_string()
    };
    register_service!(Box::new(ts));

    thread::sleep(Duration::MAX);

    mainx(); // never called
    mainy(); // never called
}

// fn register_tidal_service(ts: TidalService) {
//     println!("Registering TidalService: {:?}", ts);
//     TIDAL_SERVICES.lock().unwrap().push(ts);

//     inject_tidal_consumers();
// }

/*
fn register_service(svc: Box<dyn Any + Send + Sync>) {
    println!("Registering service: {:?}", svc);
    SERVICES.lock().unwrap().push(svc);

    inject_consumers();
}
*/

fn register_consumers() {
    let initialized = CONSUMERS_INITIALIZED.swap(true, Ordering::SeqCst);
    if initialized {
        return;
    }

    register_tidal_consumer1(|| Consumer1::default());
    register_tidal_consumer2(|| Consumer2::new());
}

fn register_tidal_consumer1(ctor: fn() -> Consumer1<'static>) {
    TIDAL_CONSUMER1.lock().unwrap().push(ctor);
}

fn register_tidal_consumer2(ctor: fn() -> Consumer2<'static>) {
    TIDAL_CONSUMER2.lock().unwrap().push(ctor);
}

fn inject_consumers() {
    for svc in SERVICES.lock().unwrap().iter() {
        inject_consumer1(&svc);
        inject_consumer2(&svc);
    }
    // inject_consumer1(ts);
    // inject_consumer2(ts);
}

fn inject_consumer1(svc: &Box<dyn Any + Send + Sync>) {
    if let Some(ts) = svc.downcast_ref::<TidalService>() {
        for ctor in TIDAL_CONSUMER1.lock().unwrap().iter() {
            let mut c = ctor();
            c.set_TidalService(ts);
            println!("c: {}", c);
        }
    }
}

fn inject_consumer2(svc: &Box<dyn Any + Send + Sync>) {
    if let Some(ts) = svc.downcast_ref::<TidalService>() {
        for ctor in TIDAL_CONSUMER2.lock().unwrap().iter() {
            let mut c = ctor();
            c.set_TidalService(ts);
            println!("c: {}", c);
        }
    }
}

fn mainy() {
    let mut sr = ServiceRegistry::new();

    let mut svcs = vec![];
    let ts = TidalService{
        location: "A".to_string()
    };
    svcs.push(ts);

    let tsref = svcs.get(0).unwrap();

    let mut c1 = Consumer1::default();
    // SERVICE_REGISTRY.register_consumer("Consumer1");
    c1.set_TidalService(tsref);
    println!("c1: {}", c1);

    thread::sleep(Duration::MAX);

    mainx(); // never called
}

fn mainx() {
    if 1 == 1 {
        return; // Ignore the rest for now
    }

    let mut sr = TestServiceRegistry::new();

    // sr.register_consumer("Consumer1", cons_fn);
    Consumer1::register_as_consumer();

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