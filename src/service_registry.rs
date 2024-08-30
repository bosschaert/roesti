use once_cell::sync::Lazy;
use std::any::Any;
use std::collections::{BTreeMap, HashMap};
use std::marker::PhantomData;
use std::sync::RwLock;
use uuid::Uuid;

pub static REGD_SERVICES: Lazy<RwLock<HashMap<ServiceRegistration,
        (Box<dyn Any + Send + Sync>, BTreeMap<String,String>)>>>
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServiceRegistration {
    pub id: Uuid,
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServiceReference<T> {
    id: Uuid,
    properties: BTreeMap<String, String>,
    _phantom: PhantomData<T>, // To give the generic signature a place
}

impl <T>ServiceReference<T> {
    pub fn from(sr: &ServiceRegistration, properties: BTreeMap<String, String>) -> ServiceReference<T> {
        ServiceReference {
            id: sr.id,
            properties,
            _phantom: PhantomData,
        }
    }

    pub fn get_properties(&self) -> &BTreeMap<String, String> {
        &self.properties
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConsumerRegistration {
    id: Uuid,
}

impl ConsumerRegistration {
    pub fn new() -> ConsumerRegistration {
        ConsumerRegistration {
            id: Uuid::new_v4(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InjectMetadata {
    fields_injected: usize,
    is_activated: bool,
}

impl InjectMetadata {
    pub fn new() -> InjectMetadata {
        InjectMetadata {
            fields_injected: 0,
            is_activated: false,
        }
    }

    pub fn inc_fields_injected(&mut self) {
        self.fields_injected += 1;
    }

    pub fn get_fields_injected(&self) -> usize {
        self.fields_injected
    }

    pub fn set_activated(&mut self) {
        self.is_activated = true;
    }

    pub fn is_activated(&self) -> bool {
        self.is_activated
    }
}