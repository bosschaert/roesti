use crate::sunlight_service::SunlightService;
use crate::tidal_service::TidalService;
use dynamic_services::ServiceReference;
use dynamic_services_derive::DynamicServices;
use dynamic_services_derive::{activator, dynamic_services, update};

#[derive(DynamicServices, Default)]
pub struct Consumer3<'a, 'b> {
    _foo: &'a str,
    _bar: &'b str,

    #[inject]
    tidal_ref_obj: Option<ServiceReference<TidalService>>,

    #[inject]
    sunlight: Option<ServiceReference<SunlightService>>
}

#[dynamic_services(path=roesti::consumer3)]
impl Consumer3<'_, '_> {
    pub fn default() -> Self {
        Consumer3 { _foo: "foo", _bar: "bar", tidal_ref_obj: None, sunlight: None }
    }

    // Called after the constructor has been called.
    #[activator]
    pub fn activate(&self, sls: &SunlightService, ts: &TidalService) {
        println!("Consumer 3 Activated:\n{:?}\n{:?}", self.tidal_ref_obj, self.sunlight);
        println!("                    :\n{:?}\n{:?}", sls.next_sundown(), ts.next_event());
        if let Some(sr) = &self.tidal_ref_obj {
            println!("  properties: {:?}", sr.get_properties());
        }
    }

    #[update]
    pub fn update(&self, field: &str, props: std::collections::BTreeMap<String, String>) {
        println!("My Consumer 3 Updated. {:?} - {:?}", field, props);
        if field == "tidal_ref_obj" {
            if let Some(sr) = &self.tidal_ref_obj {
                println!(" field properties: {:?}", sr.get_properties());
            }
        }
    }
}
