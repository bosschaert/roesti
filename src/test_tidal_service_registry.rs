use crate::tidal_service::TidalService;

struct TidalService_Registry {
  services: Vec<TidalService>,
}

impl TidalService_Registry {
  pub fn add_service(&mut self, service: TidalService) {
    self.services.push(service);
  }
}