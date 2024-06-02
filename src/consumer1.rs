use crate::tidal_service::TidalService;
use std::fmt::Display;

use dynamic_services_derive::DynamicServices;

#[derive(DynamicServices)]
pub struct Consumer1<'a> {
  blahh: u32,
  #[inject]
  tidal: Option<&'a TidalService>
}

impl Consumer1<'_> {
  pub fn new() -> Self {
    Consumer1 { tidal: None, blahh: 12 }
  }
}

impl<'a> Consumer1<'a> {
  pub fn set_tidal_service(&mut self, ts: &'a TidalService) {
    self.tidal = Some(ts);
  }
}

impl Display for Consumer1<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Consumer1 {{ tidal: {:?} - {} }}", self.tidal.is_some(), self.blahh)
  }
}