use crate::tidal_service::TidalService;
use std::fmt::Display;

use dynamic_services_derive::DynamicServices;

#[derive(DynamicServices)]
pub struct Consumer1 {
  tidal: Option<&'static TidalService>,
}

impl Consumer1 {
  pub fn new() -> Self {
    Consumer1 { tidal: None }
  }
}

impl Consumer1 {
  pub fn foo(&self) {
    println!("foo");
  }
}

impl Display for Consumer1 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Consumer1 {{ tidal: {:?} }}", self.tidal.is_some())
  }
}
