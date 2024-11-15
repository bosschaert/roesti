#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::boxed::Box;
use std::collections::BTreeMap;
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
    let sls = SunlightService {
        location: "B".to_string(),
    };
    let _sreg2 = register_service(Box::new(sls), BTreeMap::new());
    props.remove("hello");
    props.insert("hi".to_string(), "ha".to_string());
    update_service(&sreg, props);
    unregister_service(sreg);
}
fn register_service(
    svc: Box<dyn ::std::any::Any + Send + Sync>,
    mut props: std::collections::BTreeMap<String, String>,
) -> ::dynamic_services::ServiceRegistration {
    register_consumers();
    let sreg = ::dynamic_services::ServiceRegistration::new();
    props.insert(".service_id".to_string(), sreg.id.to_string());
    ::dynamic_services::REGD_SERVICES
        .write()
        .unwrap()
        .insert(sreg.clone(), (svc, props));
    inject_consumers();
    sreg
}
fn update_service(
    sreg: &::dynamic_services::ServiceRegistration,
    mut props: std::collections::BTreeMap<String, String>,
) {
    props.insert(".service_id".to_string(), sreg.id.to_string());
    {
        let mut regd = ::dynamic_services::REGD_SERVICES.write().unwrap();
        if let Some((_, p)) = regd.get_mut(sreg) {
            *p = props.clone();
        }
    }
    update_consumers(sreg, props);
}
fn unregister_service(sr: ::dynamic_services::ServiceRegistration) {
    if ::dynamic_services::REGD_SERVICES.write().unwrap().remove(&sr).is_some() {
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
            ::dynamic_services::ConsumerRegistration,
            (
                roesti::consumer1::Consumer1,
                Vec<::dynamic_services::ServiceRegistration>,
                ::dynamic_services::InjectMetadata,
            ),
        >,
    >,
> = ::once_cell::sync::Lazy::new(|| std::sync::RwLock::new(
    std::collections::HashMap::new(),
));
#[allow(non_snake_case)]
fn register_Consumer1() {
    {
        ::std::io::_print(format_args!("Registering Consumer: {0}\n", "Consumer1"));
    };
    CONSUMER_CTOR_CONSUMER1
        .write()
        .unwrap()
        .push(|| roesti::consumer1::Consumer1::default());
}
#[allow(non_snake_case)]
fn inject_Consumer1(
    svc: &Box<dyn ::std::any::Any + Send + Sync>,
    sreg: &::dynamic_services::ServiceRegistration,
    props: &std::collections::BTreeMap<String, String>,
) {
    let mut gm = CONSUMER_INST_CONSUMER1.write().unwrap();
    if gm.is_empty() {
        for ctor in CONSUMER_CTOR_CONSUMER1.read().unwrap().iter() {
            let mut i = ctor();
            let regs = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([sreg.clone()]),
            );
            gm.insert(
                ::dynamic_services::ConsumerRegistration::new(),
                (i, regs, ::dynamic_services::InjectMetadata::new()),
            );
        }
    } else {
        for ctor in CONSUMER_CTOR_CONSUMER1.read().unwrap().iter() {
            for (_, (_, regs, _)) in gm.iter_mut() {
                if !regs.contains(sreg) {
                    regs.push(sreg.clone());
                }
            }
        }
    }
    if let Some(sr) = svc.downcast_ref::<TidalService>() {
        for (_, (i, _, md)) in gm.iter_mut() {
            if i.get_TidalService_ref().is_none() {
                i.set_TidalService_ref(sreg, props);
                md.inc_fields_injected();
            }
        }
    }
    for (_, (c, regs, md)) in gm.iter_mut() {
        if md.get_fields_injected() == 1usize && !md.is_activated() {
            md.set_activated();
        }
    }
}
#[allow(non_snake_case)]
fn update_Consumer1(
    sreg: &::dynamic_services::ServiceRegistration,
    props: &std::collections::BTreeMap<String, String>,
) {
    let regd = ::dynamic_services::REGD_SERVICES.read().unwrap();
    let svc = regd.get(&sreg);
    if let Some((svcx, propsx)) = svc {
        let mut gm = CONSUMER_INST_CONSUMER1.read().unwrap();
        {
            ::std::io::_print(
                format_args!("XUpdating service: {0:?} - {1:?}\n", svcx, propsx),
            );
        };
        if let Some(dcsvc) = svcx.downcast_ref::<TidalService>() {
            {
                ::std::io::_print(format_args!("Found my service {0:?}\n", dcsvc));
            };
            for (_, (i, _, _)) in gm.iter() {
                i.update_TidalService(sreg, propsx);
            }
        }
    }
}
#[allow(non_snake_case)]
fn uninject_Consumer1(sreg: &::dynamic_services::ServiceRegistration) {
    let mut deleted = ::alloc::vec::Vec::new();
    let mut global = CONSUMER_INST_CONSUMER1.write().unwrap();
    global
        .iter_mut()
        .filter(|(_, (_, regs, _))| regs.contains(sreg))
        .for_each(|(ci, (c, _, _))| {
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
            ::dynamic_services::ConsumerRegistration,
            (
                roesti::consumer2::Consumer2,
                Vec<::dynamic_services::ServiceRegistration>,
                ::dynamic_services::InjectMetadata,
            ),
        >,
    >,
> = ::once_cell::sync::Lazy::new(|| std::sync::RwLock::new(
    std::collections::HashMap::new(),
));
#[allow(non_snake_case)]
fn register_Consumer2() {
    {
        ::std::io::_print(format_args!("Registering Consumer: {0}\n", "Consumer2"));
    };
    CONSUMER_CTOR_CONSUMER2
        .write()
        .unwrap()
        .push(|| roesti::consumer2::Consumer2::default());
}
#[allow(non_snake_case)]
fn inject_Consumer2(
    svc: &Box<dyn ::std::any::Any + Send + Sync>,
    sreg: &::dynamic_services::ServiceRegistration,
    props: &std::collections::BTreeMap<String, String>,
) {
    let mut gm = CONSUMER_INST_CONSUMER2.write().unwrap();
    if gm.is_empty() {
        for ctor in CONSUMER_CTOR_CONSUMER2.read().unwrap().iter() {
            let mut i = ctor();
            let regs = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([sreg.clone()]),
            );
            gm.insert(
                ::dynamic_services::ConsumerRegistration::new(),
                (i, regs, ::dynamic_services::InjectMetadata::new()),
            );
        }
    } else {
        for ctor in CONSUMER_CTOR_CONSUMER2.read().unwrap().iter() {
            for (_, (_, regs, _)) in gm.iter_mut() {
                if !regs.contains(sreg) {
                    regs.push(sreg.clone());
                }
            }
        }
    }
    if let Some(sr) = svc.downcast_ref::<TidalService>() {
        for (_, (i, _, md)) in gm.iter_mut() {
            if i.get_TidalService_ref().is_none() {
                i.set_TidalService_ref(sreg, props);
                md.inc_fields_injected();
            }
        }
    }
    for (_, (c, regs, md)) in gm.iter_mut() {
        if md.get_fields_injected() == 1usize && !md.is_activated() {
            let svc_registry = ::dynamic_services::REGD_SERVICES.read().unwrap();
            let mut arg0 = None;
            for reg in regs.clone() {
                let (svc, _) = svc_registry.get(&reg).unwrap();
                if let Some(sr) = svc.downcast_ref::<TidalService>() {
                    arg0 = Some(sr);
                }
            }
            if true && arg0.is_some() {
                let argval0 = arg0.unwrap();
                c.activate(argval0);
            }
            md.set_activated();
        }
    }
}
#[allow(non_snake_case)]
fn update_Consumer2(
    sreg: &::dynamic_services::ServiceRegistration,
    props: &std::collections::BTreeMap<String, String>,
) {
    let regd = ::dynamic_services::REGD_SERVICES.read().unwrap();
    let svc = regd.get(&sreg);
    if let Some((svcx, propsx)) = svc {
        let mut gm = CONSUMER_INST_CONSUMER2.read().unwrap();
        {
            ::std::io::_print(
                format_args!("XUpdating service: {0:?} - {1:?}\n", svcx, propsx),
            );
        };
        if let Some(dcsvc) = svcx.downcast_ref::<TidalService>() {
            {
                ::std::io::_print(format_args!("Found my service {0:?}\n", dcsvc));
            };
            for (_, (i, _, _)) in gm.iter() {
                i.update_TidalService(sreg, propsx);
            }
        }
    }
}
#[allow(non_snake_case)]
fn uninject_Consumer2(sreg: &::dynamic_services::ServiceRegistration) {
    let mut deleted = ::alloc::vec::Vec::new();
    let mut global = CONSUMER_INST_CONSUMER2.write().unwrap();
    global
        .iter_mut()
        .filter(|(_, (_, regs, _))| regs.contains(sreg))
        .for_each(|(ci, (c, _, _))| {
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
            ::dynamic_services::ConsumerRegistration,
            (
                roesti::consumer3::Consumer3,
                Vec<::dynamic_services::ServiceRegistration>,
                ::dynamic_services::InjectMetadata,
            ),
        >,
    >,
> = ::once_cell::sync::Lazy::new(|| std::sync::RwLock::new(
    std::collections::HashMap::new(),
));
#[allow(non_snake_case)]
fn register_Consumer3() {
    {
        ::std::io::_print(format_args!("Registering Consumer: {0}\n", "Consumer3"));
    };
    CONSUMER_CTOR_CONSUMER3
        .write()
        .unwrap()
        .push(|| roesti::consumer3::Consumer3::default());
}
#[allow(non_snake_case)]
fn inject_Consumer3(
    svc: &Box<dyn ::std::any::Any + Send + Sync>,
    sreg: &::dynamic_services::ServiceRegistration,
    props: &std::collections::BTreeMap<String, String>,
) {
    let mut gm = CONSUMER_INST_CONSUMER3.write().unwrap();
    if gm.is_empty() {
        for ctor in CONSUMER_CTOR_CONSUMER3.read().unwrap().iter() {
            let mut i = ctor();
            let regs = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([sreg.clone()]),
            );
            gm.insert(
                ::dynamic_services::ConsumerRegistration::new(),
                (i, regs, ::dynamic_services::InjectMetadata::new()),
            );
        }
    } else {
        for ctor in CONSUMER_CTOR_CONSUMER3.read().unwrap().iter() {
            for (_, (_, regs, _)) in gm.iter_mut() {
                if !regs.contains(sreg) {
                    regs.push(sreg.clone());
                }
            }
        }
    }
    if let Some(sr) = svc.downcast_ref::<TidalService>() {
        for (_, (i, _, md)) in gm.iter_mut() {
            if i.get_TidalService_ref().is_none() {
                i.set_TidalService_ref(sreg, props);
                md.inc_fields_injected();
            }
        }
    }
    if let Some(sr) = svc.downcast_ref::<SunlightService>() {
        for (_, (i, _, md)) in gm.iter_mut() {
            if i.get_SunlightService_ref().is_none() {
                i.set_SunlightService_ref(sreg, props);
                md.inc_fields_injected();
            }
        }
    }
    for (_, (c, regs, md)) in gm.iter_mut() {
        if md.get_fields_injected() == 2usize && !md.is_activated() {
            let svc_registry = ::dynamic_services::REGD_SERVICES.read().unwrap();
            let mut arg0 = None;
            for reg in regs.clone() {
                let (svc, _) = svc_registry.get(&reg).unwrap();
                if let Some(sr) = svc.downcast_ref::<SunlightService>() {
                    arg0 = Some(sr);
                }
            }
            let mut arg1 = None;
            for reg in regs.clone() {
                let (svc, _) = svc_registry.get(&reg).unwrap();
                if let Some(sr) = svc.downcast_ref::<TidalService>() {
                    arg1 = Some(sr);
                }
            }
            if true && arg0.is_some() && arg1.is_some() {
                let argval0 = arg0.unwrap();
                let argval1 = arg1.unwrap();
                c.activate(argval0, argval1);
            }
            md.set_activated();
        }
    }
}
#[allow(non_snake_case)]
fn update_Consumer3(
    sreg: &::dynamic_services::ServiceRegistration,
    props: &std::collections::BTreeMap<String, String>,
) {
    let regd = ::dynamic_services::REGD_SERVICES.read().unwrap();
    let svc = regd.get(&sreg);
    if let Some((svcx, propsx)) = svc {
        let mut gm = CONSUMER_INST_CONSUMER3.read().unwrap();
        {
            ::std::io::_print(
                format_args!("XUpdating service: {0:?} - {1:?}\n", svcx, propsx),
            );
        };
        if let Some(dcsvc) = svcx.downcast_ref::<TidalService>() {
            {
                ::std::io::_print(format_args!("Found my service {0:?}\n", dcsvc));
            };
            for (_, (i, _, _)) in gm.iter() {
                i.update_TidalService(sreg, propsx);
            }
        }
        if let Some(dcsvc) = svcx.downcast_ref::<SunlightService>() {
            {
                ::std::io::_print(format_args!("Found my service {0:?}\n", dcsvc));
            };
            for (_, (i, _, _)) in gm.iter() {
                i.update_SunlightService(sreg, propsx);
            }
        }
    }
}
#[allow(non_snake_case)]
fn uninject_Consumer3(sreg: &::dynamic_services::ServiceRegistration) {
    let mut deleted = ::alloc::vec::Vec::new();
    let mut global = CONSUMER_INST_CONSUMER3.write().unwrap();
    global
        .iter_mut()
        .filter(|(_, (_, regs, _))| regs.contains(sreg))
        .for_each(|(ci, (c, _, _))| {
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
    register_Consumer2();
    register_Consumer3();
    register_Consumer1();
}
fn inject_consumers() {
    for (sreg, (svc, props)) in ::dynamic_services::REGD_SERVICES.read().unwrap().iter()
    {
        inject_Consumer2(svc, &sreg, &props);
        inject_Consumer3(svc, &sreg, &props);
        inject_Consumer1(svc, &sreg, &props);
    }
}
fn update_consumers(
    sreg: &::dynamic_services::ServiceRegistration,
    props: std::collections::BTreeMap<String, String>,
) {
    update_Consumer2(sreg, &props);
    update_Consumer3(sreg, &props);
    update_Consumer1(sreg, &props);
}
fn uninject_consumers(sr: &::dynamic_services::ServiceRegistration) {
    uninject_Consumer2(sr);
    uninject_Consumer3(sr);
    uninject_Consumer1(sr);
}
