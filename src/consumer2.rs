use std::fmt::Display;

use crate::tidal_service::TidalService;
use crate::service_registry::{ServiceReference, ServiceRegistration, REGD_SERVICES};
use dynamic_services_derive::DynamicServices;
use dynamic_services_derive::{activator, deactivator, dynamic_services};

#[derive(DynamicServices, Debug, Default)]
pub struct Consumer2<'a> {
    // TODO change the inject to the ref
    #[inject]
    tidal: Option<&'a TidalService>,

    // TODO generate this one?
    // tidal_ref: Option<ServiceRegistration>,
    tidal_ref: Option<ServiceReference<TidalService>>,
}

#[dynamic_services]
impl <'a>Consumer2<'a> {
    // Called after the constructor has been called.
    // TODO pass in reference to service?
    #[activator]
    pub fn activate(&self, ts: &TidalService) {
        println!("Consumer 2 Activated... {} - {:?}", ts.next_event(), self.tidal_ref);
        self.invoke_tidal(|sr| {
            let ne = sr.next_event();
            println!("Custom next event: {}", ne);
        });
    }

    #[deactivator]
    pub fn deactivate(&self) {
        println!("Consumer 2 Deactivated...");
    }

    // TODO also provide an update method?

    pub fn new() -> Self {
        Consumer2 { tidal: None, tidal_ref: None }
    }
}

impl Display for Consumer2<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Consumer2 {{ tidal: {} }}", self.tidal.is_some())
  }
}
