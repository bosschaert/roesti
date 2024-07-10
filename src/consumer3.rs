use crate::tidal_service::TidalService;
use crate::service_registry::ServiceReference;
use dynamic_services_derive::DynamicServices;
use dynamic_services_derive::{activator, dynamic_services};

#[derive(DynamicServices, Debug, Default)]
pub struct Consumer3<'a, 'b> {
    _foo: &'a str,
    _bar: &'b str,

    #[inject]
    tidal_ref_obj: Option<ServiceReference<TidalService>>,
}

#[dynamic_services(path=roesti::consumer3)]
impl Consumer3<'_, '_> {
    pub fn default() -> Self {
        Consumer3 { _foo: "foo", _bar: "bar", tidal_ref_obj: None }
    }

    // Called after the constructor has been called.
    #[activator]
    pub fn activate(&self, _tidal: &TidalService) {
        println!("Consumer 3 Activated: {:?}", self.tidal_ref_obj);
    }
}
