use crate::tidal_service::TidalService;
use crate::service_registry::ServiceReference;
use dynamic_services::DynamicServices;
use dynamic_services::{activator, deactivator, dynamic_services};

#[derive(DynamicServices, Debug, Default)]
pub struct Consumer2 {
    #[inject]
    tidal: Option<ServiceReference<TidalService>>,
}

#[dynamic_services(path=roesti::consumer2)]
impl Consumer2 {
    // Called after the constructor has been called.
    // TODO pass in reference to the services requested
    #[activator]
    pub fn activate(&self, ts: &TidalService) {
        println!("Consumer 2 Activated... {} - {:?}", ts.next_event(), self.tidal);
    }

    #[deactivator]
    pub fn deactivate(&self) {
        println!("Consumer 2 Deactivated...");
    }

    // TODO also provide an update method?

    pub fn new() -> Self {
        Consumer2 { tidal: None }
    }
}
