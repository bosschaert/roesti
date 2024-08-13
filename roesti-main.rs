#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::collections::BTreeMap;
use std::time::Duration;
use std::thread;
use roesti::sunlight_service::SunlightService;
use roesti::tidal_service::TidalService;
use dynamic_services_derive::dynamic_services_main;
fn main() {
    let ts = TidalService {
        location: "A".to_string(),
    };
    let mut props = BTreeMap::new();
    props.insert("foo".to_owned(), "bar".to_owned());
    props.insert("hello".to_owned(), "123".to_owned());
    let sreg = register_service(Box::new(ts), props.clone());
    let sunlight_service = SunlightService {
        location: "B".to_string(),
    };
    let sreg2 = register_service(Box::new(sunlight_service), BTreeMap::new());
    thread::sleep(Duration::from_secs(1));
    props.remove("hello");
    props.insert("hi".to_string(), "ha".to_string());
    update_service(&sreg, props);
    unregister_service(sreg2);
    unregister_service(sreg);
}
fn register_service(
    svc: Box<dyn ::std::any::Any + Send + Sync>,
    mut props: std::collections::BTreeMap<String, String>,
) -> ::roesti::service_registry::ServiceRegistration {
    register_consumers();
    let sreg = ::roesti::service_registry::ServiceRegistration::new();
    props.insert(".service_id".to_string(), sreg.id.to_string());
    {
        ::std::io::_print(
            format_args!("Registering service: {0:?} - {1:?}\n", svc, sreg),
        );
    };
    ::roesti::service_registry::REGD_SERVICES
        .write()
        .unwrap()
        .insert(sreg.clone(), (svc, props));
    inject_consumers();
    sreg
}
fn update_service(
    sreg: &::roesti::service_registry::ServiceRegistration,
    mut props: std::collections::BTreeMap<String, String>,
) {
    props.insert(".service_id".to_string(), sreg.id.to_string());
    let mut regd = ::roesti::service_registry::REGD_SERVICES.write().unwrap();
    if let Some((_, p)) = regd.get_mut(sreg) {
        *p = props.clone();
        update_consumers(sreg, props);
    }
}
fn unregister_service(sr: ::roesti::service_registry::ServiceRegistration) {
    if ::roesti::service_registry::REGD_SERVICES.write().unwrap().remove(&sr).is_some() {
        {
            ::std::io::_print(format_args!("Service unregistered: {0:?}\n", sr));
        };
        uninject_consumers(&sr);
    }
}
static CONSUMER_CTOR_CONSUMER1: ::once_cell::sync::Lazy<
    std::sync::RwLock<Vec<fn() -> roesti::consumer1::Consumer1<'static>>>,
> = ::once_cell::sync::Lazy::new(|| std::sync::RwLock::new(Vec::new()));
static CONSUMER_INST_CONSUMER1: ::once_cell::sync::Lazy<
    std::sync::RwLock<
        std::collections::HashMap<
            ::roesti::service_registry::ConsumerRegistration,
            (
                roesti::consumer1::Consumer1,
                Vec<::roesti::service_registry::ServiceRegistration>,
            ),
        >,
    >,
> = ::once_cell::sync::Lazy::new(|| std::sync::RwLock::new(
    std::collections::HashMap::new(),
));
fn register_Consumer1() {
    {
        ::std::io::_print(format_args!("Registering Consumer: {0}\n", "Consumer1"));
    };
    CONSUMER_CTOR_CONSUMER1
        .write()
        .unwrap()
        .push(|| roesti::consumer1::Consumer1::default());
}
fn inject_Consumer1(
    svc: &Box<dyn ::std::any::Any + Send + Sync>,
    sreg: &::roesti::service_registry::ServiceRegistration,
    props: &std::collections::BTreeMap<String, String>,
) {
    if let Some(sr) = svc.downcast_ref::<TidalService>() {
        for ctor in CONSUMER_CTOR_CONSUMER1.read().unwrap().iter() {
            let mut c = ctor();
            c.set_TidalService_ref(sreg, props);
            let regs = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([sreg.clone()]),
            );
            CONSUMER_INST_CONSUMER1
                .write()
                .unwrap()
                .insert(
                    ::roesti::service_registry::ConsumerRegistration::new(),
                    (c, regs),
                );
        }
    }
}
fn update_Consumer1(
    sreg: &::roesti::service_registry::ServiceRegistration,
    props: &std::collections::BTreeMap<String, String>,
) {
    let global = CONSUMER_INST_CONSUMER1.read().unwrap();
    global
        .iter()
        .filter(|(_, (_, regs))| regs.contains(sreg))
        .for_each(|(_, (c, _))| { });
}
fn uninject_Consumer1(sreg: &::roesti::service_registry::ServiceRegistration) {
    let mut deleted = ::alloc::vec::Vec::new();
    let mut global = CONSUMER_INST_CONSUMER1.write().unwrap();
    global
        .iter_mut()
        .filter(|(_, (_, regs))| regs.contains(sreg))
        .for_each(|(ci, (c, _))| {
            deleted.push(ci.clone());
            c.unset_all();
        });
    deleted
        .iter()
        .for_each(|ci| {
            global.remove(ci);
        });
}
static CONSUMER_CTOR_CONSUMER2: ::once_cell::sync::Lazy<
    std::sync::RwLock<Vec<fn() -> roesti::consumer2::Consumer2>>,
> = ::once_cell::sync::Lazy::new(|| std::sync::RwLock::new(Vec::new()));
static CONSUMER_INST_CONSUMER2: ::once_cell::sync::Lazy<
    std::sync::RwLock<
        std::collections::HashMap<
            ::roesti::service_registry::ConsumerRegistration,
            (
                roesti::consumer2::Consumer2,
                Vec<::roesti::service_registry::ServiceRegistration>,
            ),
        >,
    >,
> = ::once_cell::sync::Lazy::new(|| std::sync::RwLock::new(
    std::collections::HashMap::new(),
));
fn register_Consumer2() {
    {
        ::std::io::_print(format_args!("Registering Consumer: {0}\n", "Consumer2"));
    };
    CONSUMER_CTOR_CONSUMER2
        .write()
        .unwrap()
        .push(|| roesti::consumer2::Consumer2::default());
}
fn inject_Consumer2(
    svc: &Box<dyn ::std::any::Any + Send + Sync>,
    sreg: &::roesti::service_registry::ServiceRegistration,
    props: &std::collections::BTreeMap<String, String>,
) {
    if let Some(sr) = svc.downcast_ref::<TidalService>() {
        for ctor in CONSUMER_CTOR_CONSUMER2.read().unwrap().iter() {
            let mut c = ctor();
            c.set_TidalService_ref(sreg, props);
            c.activate(sr);
            let regs = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([sreg.clone()]),
            );
            CONSUMER_INST_CONSUMER2
                .write()
                .unwrap()
                .insert(
                    ::roesti::service_registry::ConsumerRegistration::new(),
                    (c, regs),
                );
        }
    }
}
fn update_Consumer2(
    sreg: &::roesti::service_registry::ServiceRegistration,
    props: &std::collections::BTreeMap<String, String>,
) {
    let global = CONSUMER_INST_CONSUMER2.read().unwrap();
    global
        .iter()
        .filter(|(_, (_, regs))| regs.contains(sreg))
        .for_each(|(_, (c, _))| { });
}
fn uninject_Consumer2(sreg: &::roesti::service_registry::ServiceRegistration) {
    let mut deleted = ::alloc::vec::Vec::new();
    let mut global = CONSUMER_INST_CONSUMER2.write().unwrap();
    global
        .iter_mut()
        .filter(|(_, (_, regs))| regs.contains(sreg))
        .for_each(|(ci, (c, _))| {
            deleted.push(ci.clone());
            c.unset_all();
            c.deactivate();
        });
    deleted
        .iter()
        .for_each(|ci| {
            global.remove(ci);
        });
}
static CONSUMER_CTOR_CONSUMER3: ::once_cell::sync::Lazy<
    std::sync::RwLock<Vec<fn() -> roesti::consumer3::Consumer3<'static, 'static>>>,
> = ::once_cell::sync::Lazy::new(|| std::sync::RwLock::new(Vec::new()));
static CONSUMER_INST_CONSUMER3: ::once_cell::sync::Lazy<
    std::sync::RwLock<
        std::collections::HashMap<
            ::roesti::service_registry::ConsumerRegistration,
            (
                roesti::consumer3::Consumer3,
                Vec<::roesti::service_registry::ServiceRegistration>,
            ),
        >,
    >,
> = ::once_cell::sync::Lazy::new(|| std::sync::RwLock::new(
    std::collections::HashMap::new(),
));
fn register_Consumer3() {
    {
        ::std::io::_print(format_args!("Registering Consumer: {0}\n", "Consumer3"));
    };
    CONSUMER_CTOR_CONSUMER3
        .write()
        .unwrap()
        .push(|| roesti::consumer3::Consumer3::default());
}
fn inject_Consumer3(
    svc: &Box<dyn ::std::any::Any + Send + Sync>,
    sreg: &::roesti::service_registry::ServiceRegistration,
    props: &std::collections::BTreeMap<String, String>,
) {
    if let Some(sr) = svc.downcast_ref::<TidalService>() {
        for ctor in CONSUMER_CTOR_CONSUMER3.read().unwrap().iter() {
            let mut c = ctor();
            c.set_TidalService_ref(sreg, props);
            c.activate(sr);
            let regs = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([sreg.clone()]),
            );
            CONSUMER_INST_CONSUMER3
                .write()
                .unwrap()
                .insert(
                    ::roesti::service_registry::ConsumerRegistration::new(),
                    (c, regs),
                );
        }
    }
}
fn update_Consumer3(
    sreg: &::roesti::service_registry::ServiceRegistration,
    props: &std::collections::BTreeMap<String, String>,
) {
    let global = CONSUMER_INST_CONSUMER3.read().unwrap();
    global
        .iter()
        .filter(|(_, (_, regs))| regs.contains(sreg))
        .for_each(|(_, (c, _))| {
            c.update(props.clone());
        });
}
fn uninject_Consumer3(sreg: &::roesti::service_registry::ServiceRegistration) {
    let mut deleted = ::alloc::vec::Vec::new();
    let mut global = CONSUMER_INST_CONSUMER3.write().unwrap();
    global
        .iter_mut()
        .filter(|(_, (_, regs))| regs.contains(sreg))
        .for_each(|(ci, (c, _))| {
            deleted.push(ci.clone());
            c.unset_all();
        });
    deleted
        .iter()
        .for_each(|ci| {
            global.remove(ci);
        });
}
fn inject_Consumer3(
    svc: &Box<dyn ::std::any::Any + Send + Sync>,
    sreg: &::roesti::service_registry::ServiceRegistration,
    props: &std::collections::BTreeMap<String, String>,
) {
    if let Some(sr) = svc.downcast_ref::<SunlightService>() {
        for ctor in CONSUMER_CTOR_CONSUMER3.read().unwrap().iter() {
            let mut c = ctor();
            c.set_SunlightService_ref(sreg, props);
            c.activate(sr);
            let regs = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([sreg.clone()]),
            );
            CONSUMER_INST_CONSUMER3
                .write()
                .unwrap()
                .insert(
                    ::roesti::service_registry::ConsumerRegistration::new(),
                    (c, regs),
                );
        }
    }
}
fn update_Consumer3(
    sreg: &::roesti::service_registry::ServiceRegistration,
    props: &std::collections::BTreeMap<String, String>,
) {
    let global = CONSUMER_INST_CONSUMER3.read().unwrap();
    global
        .iter()
        .filter(|(_, (_, regs))| regs.contains(sreg))
        .for_each(|(_, (c, _))| {
            c.update(props.clone());
        });
}
fn uninject_Consumer3(sreg: &::roesti::service_registry::ServiceRegistration) {
    let mut deleted = ::alloc::vec::Vec::new();
    let mut global = CONSUMER_INST_CONSUMER3.write().unwrap();
    global
        .iter_mut()
        .filter(|(_, (_, regs))| regs.contains(sreg))
        .for_each(|(ci, (c, _))| {
            deleted.push(ci.clone());
            c.unset_all();
        });
    deleted
        .iter()
        .for_each(|ci| {
            global.remove(ci);
        });
}
static CONSUMERS_INITIALIZED: ::std::sync::atomic::AtomicBool = ::std::sync::atomic::AtomicBool::new(
    false,
);
fn register_consumers() {
    let initialized = CONSUMERS_INITIALIZED
        .swap(true, ::std::sync::atomic::Ordering::SeqCst);
    if initialized {
        return;
    }
    register_Consumer3();
    register_Consumer1();
    register_Consumer2();
}
fn inject_consumers() {
    for (sreg, (svc, props)) in ::roesti::service_registry::REGD_SERVICES
        .read()
        .unwrap()
        .iter()
    {
        inject_Consumer3(svc, &sreg, &props);
        inject_Consumer1(svc, &sreg, &props);
        inject_Consumer2(svc, &sreg, &props);
    }
}
fn update_consumers(
    sreg: &::roesti::service_registry::ServiceRegistration,
    props: std::collections::BTreeMap<String, String>,
) {
    update_Consumer3(sreg, &props);
    update_Consumer1(sreg, &props);
    update_Consumer2(sreg, &props);
}
fn uninject_consumers(sr: &::roesti::service_registry::ServiceRegistration) {
    uninject_Consumer3(sr);
    uninject_Consumer1(sr);
    uninject_Consumer2(sr);
}
