use once_cell::sync::Lazy;
use crate::tidal_service::TidalService;
use std::fmt::Display;

use dynamic_services_derive::dynamic_services;
use dynamic_services_derive::DynamicServices;


pub static INITED: Lazy<u16> = Lazy::new(||static_init());

fn static_init() -> u16 {
  println!("**** INIT CALL");
  12
}

#[derive(DynamicServices, Debug, Default)]
pub struct Consumer1<'a> {
  blahh: u32,
  #[inject]
  tidal: Option<&'a TidalService>
}

#[dynamic_services]
impl Consumer1<'_> {
  pub fn default() -> Self {
    Consumer1 { blahh: 12, tidal: None }
  }

  pub fn register_as_consumer() {
    //   crate::service_registry::SERVICE_REGISTRY.register_consumer("TidalService", || Consumer1::default());
  }
}

impl Display for Consumer1<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Consumer1 {{ tidal: {:?} - {} }}", self.tidal.is_some(), self.blahh)
  }
}

