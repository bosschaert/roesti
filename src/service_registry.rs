use once_cell::sync::Lazy;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

pub static SERVICE_REGISTRY: Lazy<ServiceRegistry> = Lazy::new(||ServiceRegistry::new());

pub struct ServiceRegistry {
    id_counter: Mutex<u32>,
    // consumers: Mutex<HashMap<u32, (Vec<String>, Box<dyn Fn() + Send + Sync -> Box<dyn Any + Send + Sync>>)>>,
    services: Mutex<HashMap<u32, (String, Box<dyn Any + Send + Sync>)>>
}

impl ServiceRegistry {
    pub fn new() -> ServiceRegistry {
        ServiceRegistry {
            id_counter: Mutex::new(0),
            // consumers: Mutex::new(HashMap::new()),
            services: Mutex::new(HashMap::new())
        }
    }

    pub fn get_next_id(&self) -> u32 {
        let mut id = self.id_counter.lock().unwrap();
        *id += 1;
        *id
    }

    pub fn register_service(&self, name: &str, svc: Box<dyn Any + Send + Sync>) {
        println!("~~~ Registering service: {:?}", svc);
        let mut_svcs = &mut self.services.lock().unwrap();
        mut_svcs.insert(self.get_next_id(),
            (name.to_string(), svc));
        println!("    known svcs: {:?}", mut_svcs);
        }

    // pub fn xregister_consumer(&self, consumer: &str) {
    //     println!("~~~ Registering consumer: {:?}", consumer);
    //     let mut mut_cons = self.consumers.lock().unwrap();
    //     mut_cons.insert(consumer.to_string());
    //     println!("    known cons: {:?}", mut_cons);
    // }

    // pub fn register_consumer<F>(&self, deps: Vec<String>, ctor: F)
    //     where F: Fn() -> Box<dyn Any> {
    //     self.consumers.insert(self.get_next_id(),
    //         (deps, ctor)
    //     );
    // }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServiceRegistration {
    id: Uuid
}

impl ServiceRegistration {
    pub fn new() -> ServiceRegistration {
        ServiceRegistration {
            id: Uuid::new_v4()
        }
    }
}