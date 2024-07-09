use crate::service_registry::ServiceReference;
use crate::tidal_service::TidalService;
use std::fmt::Display;

use dynamic_services_derive::dynamic_services;
use dynamic_services_derive::DynamicServices;

#[derive(DynamicServices, Debug, Default)]
pub struct Consumer1<'a> {
  blahh: u32,
  _tidal: Option<&'a TidalService>,

  #[inject]
  tidal_ref: Option<ServiceReference<TidalService>>,
}

#[dynamic_services]
impl Consumer1<'_> {
  pub fn default() -> Self {
    Consumer1 { blahh: 12, _tidal: None, tidal_ref: None }
  }
}

impl Display for Consumer1<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Consumer1 {{ tidal: {:?} - {} }}", self.tidal_ref.is_some(), self.blahh)
  }
}

