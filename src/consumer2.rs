use crate::tidal_service::TidalService;
use dynamic_services::ServiceReference;
use dynamic_services_derive::DynamicServices;
use dynamic_services_derive::{activator, deactivator, dynamic_services};

#[derive(DynamicServices, Debug, Default)]
pub struct Consumer2 {
    #[inject]
    tidal: Option<ServiceReference<TidalService>>,
}

#[dynamic_services(path=roesti::consumer2)]
impl Consumer2 {
    // Called after the constructor has been called.
    #[activator]
    pub fn activate(&self, ts: &TidalService) {
        println!("Consumer 2 Activated... {} - {:?}", ts.next_event(), self.tidal);
    }

    #[deactivator]
    pub fn deactivate(&self) {
        println!("Consumer 2 Deactivated...");
        // TODO actually destruct the instance
    }
}
