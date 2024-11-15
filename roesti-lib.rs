#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod location {
    pub trait Location {
        fn location(&self) -> &str;
    }
}
pub mod tidal_service {
    pub struct TidalService {
        pub location: String,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TidalService {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TidalService {
        #[inline]
        fn eq(&self, other: &TidalService) -> bool {
            self.location == other.location
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for TidalService {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<String>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for TidalService {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.location, state)
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TidalService {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "TidalService",
                "location",
                &&self.location,
            )
        }
    }
    impl TidalService {
        pub fn location(&self) -> &str {
            &self.location
        }
        pub fn next_event(&self) -> u16 {
            42
        }
    }
}
pub mod sunlight_service {
    use crate::location::Location;
    pub struct SunlightService {
        pub location: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SunlightService {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "SunlightService",
                "location",
                &&self.location,
            )
        }
    }
    impl SunlightService {
        pub fn next_sundown(&self) -> u16 {
            22
        }
        pub fn next_sunrise(&self) -> u16 {
            8
        }
    }
    impl Location for SunlightService {
        fn location(&self) -> &str {
            &self.location
        }
    }
}
pub mod consumer1 {
    use crate::tidal_service::TidalService;
    use std::fmt::Display;
    use dynamic_services::ServiceReference;
    use dynamic_services_derive::{DynamicServices, dynamic_services};
    pub struct Consumer1<'a> {
        blahh: u32,
        _tidal: Option<&'a TidalService>,
        #[inject]
        tidal_ref: Option<ServiceReference<TidalService>>,
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for Consumer1<'a> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Consumer1",
                "blahh",
                &self.blahh,
                "_tidal",
                &self._tidal,
                "tidal_ref",
                &&self.tidal_ref,
            )
        }
    }
    #[automatically_derived]
    impl<'a> ::core::default::Default for Consumer1<'a> {
        #[inline]
        fn default() -> Consumer1<'a> {
            Consumer1 {
                blahh: ::core::default::Default::default(),
                _tidal: ::core::default::Default::default(),
                tidal_ref: ::core::default::Default::default(),
            }
        }
    }
    impl Consumer1<'_> {
        pub fn default() -> Self {
            Consumer1 {
                blahh: 12,
                _tidal: None,
                tidal_ref: None,
            }
        }
    }
    impl Consumer1<'_> {
        #[allow(non_snake_case)]
        pub fn get_TidalService_ref(&self) -> &Option<ServiceReference<TidalService>> {
            &self.tidal_ref
        }
        #[allow(non_snake_case)]
        pub fn get_TidalService_fieldname() -> &'static str {
            "tidal_ref"
        }
        #[allow(non_snake_case)]
        pub fn set_TidalService_ref(
            &mut self,
            sreg: &::dynamic_services::ServiceRegistration,
            props: &std::collections::BTreeMap<String, String>,
        ) {
            {
                ::std::io::_print(
                    format_args!(
                        "[{0}] Setting {1} to {2:?}\n",
                        "Consumer1",
                        "tidal_ref",
                        sreg,
                    ),
                );
            };
            self.tidal_ref = Some(ServiceReference::from(sreg, props.clone()));
        }
        pub fn update_TidalService(
            &mut self,
            sreg: &::dynamic_services::ServiceRegistration,
            props: &std::collections::BTreeMap<String, String>,
        ) {
            {
                ::std::io::_print(
                    format_args!(
                        "********* updating {0} with {1:?}\n",
                        "tidal_ref",
                        props,
                    ),
                );
            };
            self.set_TidalService_ref(&sreg, &props);
        }
    }
    impl Consumer1<'_> {
        pub fn unset_all(&mut self) {
            {
                ::std::io::_print(
                    format_args!("[{0}] Unsetting all injected fields\n", "Consumer1"),
                );
            };
            self.tidal_ref = None;
        }
    }
    impl Display for Consumer1<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(
                format_args!(
                    "Consumer1 {{ tidal: {0:?} - {1} }}",
                    self.tidal_ref.is_some(),
                    self.blahh,
                ),
            )
        }
    }
}
pub mod consumer2 {
    use crate::tidal_service::TidalService;
    use dynamic_services::ServiceReference;
    use dynamic_services_derive::DynamicServices;
    use dynamic_services_derive::{activator, deactivator, dynamic_services};
    pub struct Consumer2 {
        #[inject]
        tidal: Option<ServiceReference<TidalService>>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Consumer2 {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Consumer2",
                "tidal",
                &&self.tidal,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Consumer2 {
        #[inline]
        fn default() -> Consumer2 {
            Consumer2 {
                tidal: ::core::default::Default::default(),
            }
        }
    }
    impl Consumer2 {
        pub fn activate(&self, ts: &TidalService) {
            {
                ::std::io::_print(
                    format_args!(
                        "Consumer 2 Activated... {0} - {1:?}\n",
                        ts.next_event(),
                        self.tidal,
                    ),
                );
            };
        }
        pub fn deactivate(&self) {
            {
                ::std::io::_print(format_args!("Consumer 2 Deactivated...\n"));
            };
        }
    }
    impl Consumer2 {
        #[allow(non_snake_case)]
        pub fn get_TidalService_ref(&self) -> &Option<ServiceReference<TidalService>> {
            &self.tidal
        }
        #[allow(non_snake_case)]
        pub fn get_TidalService_fieldname() -> &'static str {
            "tidal"
        }
        #[allow(non_snake_case)]
        pub fn set_TidalService_ref(
            &mut self,
            sreg: &::dynamic_services::ServiceRegistration,
            props: &std::collections::BTreeMap<String, String>,
        ) {
            {
                ::std::io::_print(
                    format_args!(
                        "[{0}] Setting {1} to {2:?}\n",
                        "Consumer2",
                        "tidal",
                        sreg,
                    ),
                );
            };
            self.tidal = Some(ServiceReference::from(sreg, props.clone()));
        }
        pub fn update_TidalService(
            &mut self,
            sreg: &::dynamic_services::ServiceRegistration,
            props: &std::collections::BTreeMap<String, String>,
        ) {
            {
                ::std::io::_print(
                    format_args!("********* updating {0} with {1:?}\n", "tidal", props),
                );
            };
            self.set_TidalService_ref(&sreg, &props);
        }
    }
    impl Consumer2 {
        pub fn unset_all(&mut self) {
            {
                ::std::io::_print(
                    format_args!("[{0}] Unsetting all injected fields\n", "Consumer2"),
                );
            };
            self.tidal = None;
        }
    }
}
pub mod consumer3 {
    use crate::sunlight_service::SunlightService;
    use crate::tidal_service::TidalService;
    use dynamic_services::ServiceReference;
    use dynamic_services_derive::DynamicServices;
    use dynamic_services_derive::{activator, dynamic_services, update};
    pub struct Consumer3<'a, 'b> {
        _foo: &'a str,
        _bar: &'b str,
        #[inject]
        tidal_ref_obj: Option<ServiceReference<TidalService>>,
        #[inject]
        sunlight: Option<ServiceReference<SunlightService>>,
    }
    #[automatically_derived]
    impl<'a, 'b> ::core::default::Default for Consumer3<'a, 'b> {
        #[inline]
        fn default() -> Consumer3<'a, 'b> {
            Consumer3 {
                _foo: ::core::default::Default::default(),
                _bar: ::core::default::Default::default(),
                tidal_ref_obj: ::core::default::Default::default(),
                sunlight: ::core::default::Default::default(),
            }
        }
    }
    impl Consumer3<'_, '_> {
        pub fn default() -> Self {
            Consumer3 {
                _foo: "foo",
                _bar: "bar",
                tidal_ref_obj: None,
                sunlight: None,
            }
        }
        pub fn activate(&self, sls: &SunlightService, ts: &TidalService) {
            {
                ::std::io::_print(
                    format_args!(
                        "Consumer 3 Activated:\n{0:?}\n{1:?}\n",
                        self.tidal_ref_obj,
                        self.sunlight,
                    ),
                );
            };
            {
                ::std::io::_print(
                    format_args!(
                        "                    :\n{0:?}\n{1:?}\n",
                        sls.next_sundown(),
                        ts.next_event(),
                    ),
                );
            };
            if let Some(sr) = &self.tidal_ref_obj {
                {
                    ::std::io::_print(
                        format_args!("  properties: {0:?}\n", sr.get_properties()),
                    );
                };
            }
        }
        pub fn update(
            &self,
            field: &str,
            props: std::collections::BTreeMap<String, String>,
        ) {
            {
                ::std::io::_print(
                    format_args!("My Consumer 3 Updated. {0:?} - {1:?}\n", field, props),
                );
            };
            if field == "tidal_ref_obj" {
                if let Some(sr) = &self.tidal_ref_obj {
                    {
                        ::std::io::_print(
                            format_args!(
                                " field properties: {0:?}\n",
                                sr.get_properties(),
                            ),
                        );
                    };
                }
            }
        }
    }
    impl Consumer3<'_, '_> {
        #[allow(non_snake_case)]
        pub fn get_TidalService_ref(&self) -> &Option<ServiceReference<TidalService>> {
            &self.tidal_ref_obj
        }
        #[allow(non_snake_case)]
        pub fn get_TidalService_fieldname() -> &'static str {
            "tidal_ref_obj"
        }
        #[allow(non_snake_case)]
        pub fn set_TidalService_ref(
            &mut self,
            sreg: &::dynamic_services::ServiceRegistration,
            props: &std::collections::BTreeMap<String, String>,
        ) {
            {
                ::std::io::_print(
                    format_args!(
                        "[{0}] Setting {1} to {2:?}\n",
                        "Consumer3",
                        "tidal_ref_obj",
                        sreg,
                    ),
                );
            };
            self.tidal_ref_obj = Some(ServiceReference::from(sreg, props.clone()));
        }
        pub fn update_TidalService(
            &mut self,
            sreg: &::dynamic_services::ServiceRegistration,
            props: &std::collections::BTreeMap<String, String>,
        ) {
            {
                ::std::io::_print(
                    format_args!(
                        "********* updating {0} with {1:?}\n",
                        "tidal_ref_obj",
                        props,
                    ),
                );
            };
            self.set_TidalService_ref(&sreg, &props);
            self.update("tidal_ref_obj", props.clone());
        }
    }
    impl Consumer3<'_, '_> {
        #[allow(non_snake_case)]
        pub fn get_SunlightService_ref(
            &self,
        ) -> &Option<ServiceReference<SunlightService>> {
            &self.sunlight
        }
        #[allow(non_snake_case)]
        pub fn get_SunlightService_fieldname() -> &'static str {
            "sunlight"
        }
        #[allow(non_snake_case)]
        pub fn set_SunlightService_ref(
            &mut self,
            sreg: &::dynamic_services::ServiceRegistration,
            props: &std::collections::BTreeMap<String, String>,
        ) {
            {
                ::std::io::_print(
                    format_args!(
                        "[{0}] Setting {1} to {2:?}\n",
                        "Consumer3",
                        "sunlight",
                        sreg,
                    ),
                );
            };
            self.sunlight = Some(ServiceReference::from(sreg, props.clone()));
        }
        pub fn update_SunlightService(
            &mut self,
            sreg: &::dynamic_services::ServiceRegistration,
            props: &std::collections::BTreeMap<String, String>,
        ) {
            {
                ::std::io::_print(
                    format_args!(
                        "********* updating {0} with {1:?}\n",
                        "sunlight",
                        props,
                    ),
                );
            };
            self.set_SunlightService_ref(&sreg, &props);
            self.update("sunlight", props.clone());
        }
    }
    impl Consumer3<'_, '_> {
        pub fn unset_all(&mut self) {
            {
                ::std::io::_print(
                    format_args!("[{0}] Unsetting all injected fields\n", "Consumer3"),
                );
            };
            self.tidal_ref_obj = None;
            self.sunlight = None;
        }
    }
}
