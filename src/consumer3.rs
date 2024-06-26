use crate::tidal_service::TidalService;
use dynamic_services_derive::DynamicServices;
use dynamic_services_derive::{activator, dynamic_services};

#[derive(DynamicServices, Debug)]
pub struct Consumer3<'a> {
    #[inject]
    tidal_ref_obj: &'a TidalService
}

#[dynamic_services]
impl Consumer3<'_> {
    // Called after the constructor has been called.
    #[activator]
    pub fn activate(&self) {
        println!("Consumer 3 Activated: {:?}", self.tidal_ref_obj);
    }
}
// #[derive(DynamicServices)]
// impl <'a>Consumer2<'a> {
//     #[constructor]
//     pub fn new(ts: &'a TidalService) -> Self {
//         Consumer2 { tidal_ref_obj: ts }
//     }
// }