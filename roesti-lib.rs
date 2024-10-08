#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod service_registry {
    use once_cell::sync::Lazy;
    use std::any::Any;
    use std::collections::{BTreeMap, HashMap};
    use std::marker::PhantomData;
    use std::sync::RwLock;
    use uuid::Uuid;
    pub static REGD_SERVICES: Lazy<
        RwLock<
            HashMap<
                ServiceRegistration,
                (Box<dyn Any + Send + Sync>, BTreeMap<String, String>),
            >,
        >,
    > = Lazy::new(|| RwLock::new(HashMap::new()));
    pub struct ServiceRegistry {}
    impl ServiceRegistry {
        pub fn new() -> ServiceRegistry {
            ServiceRegistry {}
        }
    }
    pub struct ServiceRegistration {
        pub id: Uuid,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ServiceRegistration {
        #[inline]
        fn clone(&self) -> ServiceRegistration {
            ServiceRegistration {
                id: ::core::clone::Clone::clone(&self.id),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ServiceRegistration {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ServiceRegistration",
                "id",
                &&self.id,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ServiceRegistration {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ServiceRegistration {
        #[inline]
        fn eq(&self, other: &ServiceRegistration) -> bool {
            self.id == other.id
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ServiceRegistration {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Uuid>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ServiceRegistration {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ServiceRegistration,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.id, &other.id)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ServiceRegistration {
        #[inline]
        fn cmp(&self, other: &ServiceRegistration) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.id, &other.id)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for ServiceRegistration {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.id, state)
        }
    }
    impl ServiceRegistration {
        pub fn new() -> ServiceRegistration {
            ServiceRegistration {
                id: Uuid::new_v4(),
            }
        }
        pub fn from<T>(sr: &ServiceReference<T>) -> ServiceRegistration {
            ServiceRegistration { id: sr.id }
        }
    }
    pub struct ServiceReference<T> {
        id: Uuid,
        properties: BTreeMap<String, String>,
        _phantom: PhantomData<T>,
    }
    #[automatically_derived]
    impl<T: ::core::clone::Clone> ::core::clone::Clone for ServiceReference<T> {
        #[inline]
        fn clone(&self) -> ServiceReference<T> {
            ServiceReference {
                id: ::core::clone::Clone::clone(&self.id),
                properties: ::core::clone::Clone::clone(&self.properties),
                _phantom: ::core::clone::Clone::clone(&self._phantom),
            }
        }
    }
    #[automatically_derived]
    impl<T: ::core::fmt::Debug> ::core::fmt::Debug for ServiceReference<T> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "ServiceReference",
                "id",
                &self.id,
                "properties",
                &self.properties,
                "_phantom",
                &&self._phantom,
            )
        }
    }
    #[automatically_derived]
    impl<T> ::core::marker::StructuralPartialEq for ServiceReference<T> {}
    #[automatically_derived]
    impl<T: ::core::cmp::PartialEq> ::core::cmp::PartialEq for ServiceReference<T> {
        #[inline]
        fn eq(&self, other: &ServiceReference<T>) -> bool {
            self.id == other.id && self.properties == other.properties
                && self._phantom == other._phantom
        }
    }
    #[automatically_derived]
    impl<T: ::core::cmp::Eq> ::core::cmp::Eq for ServiceReference<T> {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Uuid>;
            let _: ::core::cmp::AssertParamIsEq<BTreeMap<String, String>>;
            let _: ::core::cmp::AssertParamIsEq<PhantomData<T>>;
        }
    }
    #[automatically_derived]
    impl<T: ::core::cmp::PartialOrd> ::core::cmp::PartialOrd for ServiceReference<T> {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ServiceReference<T>,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.id, &other.id) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    match ::core::cmp::PartialOrd::partial_cmp(
                        &self.properties,
                        &other.properties,
                    ) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            ::core::cmp::PartialOrd::partial_cmp(
                                &self._phantom,
                                &other._phantom,
                            )
                        }
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl<T: ::core::cmp::Ord> ::core::cmp::Ord for ServiceReference<T> {
        #[inline]
        fn cmp(&self, other: &ServiceReference<T>) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.id, &other.id) {
                ::core::cmp::Ordering::Equal => {
                    match ::core::cmp::Ord::cmp(&self.properties, &other.properties) {
                        ::core::cmp::Ordering::Equal => {
                            ::core::cmp::Ord::cmp(&self._phantom, &other._phantom)
                        }
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl<T: ::core::hash::Hash> ::core::hash::Hash for ServiceReference<T> {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.id, state);
            ::core::hash::Hash::hash(&self.properties, state);
            ::core::hash::Hash::hash(&self._phantom, state)
        }
    }
    impl<T> ServiceReference<T> {
        pub fn from(
            sr: &ServiceRegistration,
            properties: BTreeMap<String, String>,
        ) -> ServiceReference<T> {
            ServiceReference {
                id: sr.id,
                properties,
                _phantom: PhantomData,
            }
        }
        pub fn get_properties(&self) -> &BTreeMap<String, String> {
            &self.properties
        }
    }
    pub struct ConsumerRegistration {
        id: Uuid,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConsumerRegistration {
        #[inline]
        fn clone(&self) -> ConsumerRegistration {
            let _: ::core::clone::AssertParamIsClone<Uuid>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ConsumerRegistration {}
    #[automatically_derived]
    impl ::core::fmt::Debug for ConsumerRegistration {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ConsumerRegistration",
                "id",
                &&self.id,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ConsumerRegistration {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ConsumerRegistration {
        #[inline]
        fn eq(&self, other: &ConsumerRegistration) -> bool {
            self.id == other.id
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ConsumerRegistration {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Uuid>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ConsumerRegistration {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ConsumerRegistration,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.id, &other.id)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ConsumerRegistration {
        #[inline]
        fn cmp(&self, other: &ConsumerRegistration) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.id, &other.id)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for ConsumerRegistration {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.id, state)
        }
    }
    impl ConsumerRegistration {
        pub fn new() -> ConsumerRegistration {
            ConsumerRegistration {
                id: Uuid::new_v4(),
            }
        }
    }
}
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
    use crate::service_registry::ServiceReference;
    use crate::tidal_service::TidalService;
    use std::fmt::Display;
    use dynamic_services_derive::dynamic_services;
    use dynamic_services_derive::DynamicServices;
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
        pub fn set_TidalService_ref(
            &mut self,
            sreg: &crate::service_registry::ServiceRegistration,
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
    use crate::service_registry::ServiceReference;
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
        pub fn new() -> Self {
            Consumer2 { tidal: None }
        }
    }
    impl Consumer2 {
        pub fn set_TidalService_ref(
            &mut self,
            sreg: &crate::service_registry::ServiceRegistration,
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
    use crate::service_registry::ServiceReference;
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
    impl<'a, 'b> ::core::fmt::Debug for Consumer3<'a, 'b> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Consumer3",
                "_foo",
                &self._foo,
                "_bar",
                &self._bar,
                "tidal_ref_obj",
                &self.tidal_ref_obj,
                "sunlight",
                &&self.sunlight,
            )
        }
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
        pub fn activate(&self, _tidal: &TidalService) {
            {
                ::std::io::_print(
                    format_args!("Consumer 3 Activated: {0:?}\n", self.tidal_ref_obj),
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
        pub fn update(&self, props: std::collections::BTreeMap<String, String>) {
            {
                ::std::io::_print(format_args!("Consumer 3 Updated: {0:?}\n", props));
            };
        }
    }
    impl Consumer3<'_, '_> {
        pub fn set_TidalService_ref(
            &mut self,
            sreg: &crate::service_registry::ServiceRegistration,
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
    }
    impl Consumer3<'_, '_> {
        pub fn set_SunlightService_ref(
            &mut self,
            sreg: &crate::service_registry::ServiceRegistration,
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
