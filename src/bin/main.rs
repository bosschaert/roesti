use once_cell::sync::Lazy;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::thread;
use roesti::{test_service_registry::TestServiceRegistry, sunlight_service::SunlightService, tidal_service::TidalService, location::Location};
use roesti::consumer1::Consumer1;
use roesti::consumer2::Consumer2;
use roesti::service_registry::{ConsumerRegistration, ServiceRegistration, ServiceRegistry, REGD_SERVICES};

use dynamic_services_derive::dynamic_services_main;

// Probably not needed as a macro_rules
macro_rules! register_service {
    ($svc:expr) => {
        register_service($svc)
    };
}

static CONSUMER_INST_CONSUMERX: Lazy<
    Mutex<HashMap<ConsumerRegistration, (Consumer1, Vec<ServiceRegistration>)>>,
> = Lazy::new(|| Mutex::new(HashMap::new()));

#[dynamic_services_main]
fn main() {
    let ts = TidalService{
        location: "A".to_string()
    };
    let sreg = register_service!(Box::new(ts));

    thread::sleep(Duration::from_secs(1));
    unregister_service(sreg);

    let mut deleted = vec![];
    let mut cim = CONSUMER_INST_CONSUMERX.lock().unwrap();
    cim.iter_mut()
        .filter(|(_, (_, regs))| regs.len() > 0)
        .for_each(|(ci, (c, _))| {
            // let (c, _) = cr;
            c.unset_all();
            deleted.push(ci.clone());
        });
    // cim.retain(f)

    deleted.iter().for_each(|ci| { cim.remove(ci); } );
    // cim
    // let mut keyrefs = Vec::new();
    // {
    //     let mut m = CONSUMER_INST_CONSUMERX.lock().unwrap();
    //     m.keys().for_each(|k| {
    //         keyrefs.push(k);
    //     });
    // }
    // {
    //     let mut m = CONSUMER_INST_CONSUMERX.lock().unwrap();
    //     keyrefs.iter().for_each(|k| {
    //         m.remove(k);
    //     });
    // }
    // keyrefs.iter().for_each(|k| {
    //     m.remove(k);
    // });
    // m.
    // let mut keys = m.keys();

    thread::sleep(Duration::MAX);



    // let mut m = CONSUMER_INST_CONSUMER2.lock().unwrap();
    // let mut keys = m.keys();
    // for mut k in keys {
    //     // if let Some(mut mk) = m.get_mut(k) {
    //         k.unset_all();
    //     // }
    // }

    // // let mut i: (&mut Consumer2, Vec<ServiceRegistration>) = m.iter_mut();
    // let mut i: std::collections::hash_map::IterMut<'_, &mut Consumer2<'static>, Vec<ServiceRegistration>> = m.iter_mut();
    // i.for_each(|(c, _)| {
    //     c.unset_all();
    // });
    // i.filter(|(_, regs)| regs.len() > 0).for_each(|(cons, _)| {
    //     cons.unset_all();
    // });

    // -------------------------------------------
    mainx(); // never called
    mainy(); // never called
}

// -------------------------------------------

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
    // Consumer1::register_as_consumer();

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