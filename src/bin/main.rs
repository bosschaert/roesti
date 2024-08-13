use std::collections::BTreeMap;
use std::time::Duration;
use std::thread;
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

    thread::sleep(Duration::from_secs(1));

    props.remove("hello");
    props.insert("hi".to_string(), "ha".to_string());
    update_service(&sreg, props);

    unregister_service(sreg);

    // TODO add a different service
}

fn inject_cons() {
    let svcs = ::roesti::service_registry::REGD_SERVICES.read().unwrap();
    inject_Consumer3a(svcs);
}

fn inject_Consumer3a(svcs: std::sync::RwLockReadGuard<std::collections::HashMap<roesti::service_registry::ServiceRegistration, (Box<dyn std::any::Any + Send + Sync>, BTreeMap<String, String>)>>) {
    for ctor in CONSUMER_CTOR_CONSUMER3.read().unwrap().iter() {        
        let mut ts = None;
        let mut ss = None;

        for (sreg, (svc, props)) in svcs.iter() {
            if let Some(sr) = svc.downcast_ref::<TidalService>() {
                ts = Some((sr, sreg, props));
            }
            if let Some(sr) = svc.downcast_ref::<SunlightService>() {
                ss = Some((sr, sreg, props));
            }
        }

        if let Some((tss, tsssreg, tssprops)) = ts {
            if let Some((sss, ssssreg, sssprops)) = ss {
                let mut c = ctor();

                c.set_TidalService_ref(tsssreg, tssprops);
                c.set_SunlightService_ref(ssssreg, sssprops);

                c.activate(tss /*, sss */);

                let regs = vec![sreg.clone()];
                CONSUMER_INST_CONSUMER3.write().unwrap().insert(
                    ::roesti::service_registry::ConsumerRegistration::new(), (c, regs));
            }
        }


        // if let Some(sr) = svc.downcast_ref::<TidalService>() {
        //     for ctor in CONSUMER_CTOR_CONSUMER3.read().unwrap().iter() {
        //         let mut c = ctor();
        //         c.set_TidalService_ref(sreg, props);
        //         c.activate(sr);
        //         let regs = <[_]>::into_vec(
        //             #[rustc_box]
        //             ::alloc::boxed::Box::new([sreg.clone()]),
        //         );
        //         CONSUMER_INST_CONSUMER3
        //             .write()
        //             .unwrap()
        //             .insert(
        //                 ::roesti::service_registry::ConsumerRegistration::new(),
        //                 (c, regs),
        //             );
        //     }
        // }
    

    }
}


