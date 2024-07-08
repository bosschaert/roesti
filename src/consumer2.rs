use std::fmt::Display;

use crate::tidal_service::TidalService;
use crate::service_registry::{ServiceRegistration, REGD_SERVICES};
use dynamic_services_derive::DynamicServices;
use dynamic_services_derive::{activator, dynamic_services};

#[derive(DynamicServices, PartialEq, Eq, Hash, Debug, Default)]
pub struct Consumer2<'a> {
    #[inject]
    tidal: Option<&'a TidalService>,
    tidal_ref: Option<ServiceRegistration>,
}

#[dynamic_services]
impl <'a>Consumer2<'a> {
    // Called after the constructor has been called.
    #[activator]
    pub fn activate(&self) {
        println!("Consumer 2 Activated...");
        self.invoke_tidal(|sr| {
          let ne = sr.next_event();
          println!("next event: {}", ne);
        });
        println!("Attempted invoke");
    }

    pub fn new() -> Self {
        Consumer2 { tidal: None, tidal_ref: None }
    }

    // fn invoke_tidal(&self, ts: impl Fn(&TidalService)) {
    //   let sr = REGD_SERVICES.lock().unwrap();
    //   let svc = sr.get(&self.tidal_ref.unwrap()).unwrap();
    //   if let Some(sr) = svc.downcast_ref::<TidalService>() {
    //     ts(sr);
    //   }
    // }

    // pub fn set_TidalService_ref(&mut self, sr: ServiceRegistration) {
    //   self.tidal_ref = Some(sr);
    // }
}

impl Display for Consumer2<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Consumer2 {{ tidal: {} }}", self.tidal.is_some())
  }
}
