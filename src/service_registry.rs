use once_cell::sync::Lazy;
use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::RwLock;
use uuid::Uuid;

pub static REGD_SERVICES: Lazy<RwLock<HashMap<ServiceRegistration, Box<dyn Any + Send + Sync>>>>
    = Lazy::new(||RwLock::new(HashMap::new()));

pub struct ServiceRegistry {
    // id_counter: Mutex<u32>,
}

impl ServiceRegistry {
    pub fn new() -> ServiceRegistry {
        ServiceRegistry {
            // id_counter: Mutex::new(0),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServiceRegistration {
    id: Uuid,
}

impl ServiceRegistration {
    pub fn new() -> ServiceRegistration {
        ServiceRegistration {
            id: Uuid::new_v4(),
        }
    }

    pub fn from<T>(sr: &ServiceReference<T>) -> ServiceRegistration {
        ServiceRegistration {
            id: sr.id,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServiceReference<T> {
    id: Uuid,
    _phantom: PhantomData<T>,
}

impl <T>ServiceReference<T> {
    pub fn from(sr: &ServiceRegistration) -> ServiceReference<T> {
        ServiceReference {
            id: sr.id,
            _phantom: PhantomData,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConsumerRegistration {
    id: Uuid
}

impl ConsumerRegistration {
    pub fn new() -> ConsumerRegistration {
        ConsumerRegistration {
            id: Uuid::new_v4()
        }
    }
}