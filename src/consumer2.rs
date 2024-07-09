use std::fmt::Display;

use crate::tidal_service::TidalService;
use crate::service_registry::{ServiceReference, ServiceRegistration, REGD_SERVICES};
use dynamic_services_derive::DynamicServices;
use dynamic_services_derive::{activator, deactivator, dynamic_services};

#[derive(DynamicServices, Debug, Default)]
pub struct Consumer2 {
    // // TODO change the inject to the ref
    // tidal: Option<&'a TidalService>,

    #[inject]
    tidal: Option<ServiceReference<TidalService>>,
}

#[dynamic_services]
impl Consumer2 {
    // Called after the constructor has been called.
    // TODO pass in reference to service?
    #[activator]
    pub fn activate(&self, ts: &TidalService) {
        println!("Consumer 2 Activated... {} - {:?}", ts.next_event(), self.tidal);
        // self.invoke_tidal(|sr| {
        //     let ne = sr.next_event();
        //     println!("Custom next event: {}", ne);
        // });
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
