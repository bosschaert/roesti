use std::any::Any;
use std::collections::{BTreeMap, HashMap};

pub struct ServiceRegistry {
    id_counter: u32,
    services: HashMap<BTreeMap<String, String>, Box<dyn Any>>,
}

impl ServiceRegistry {
    pub fn new() -> ServiceRegistry {
        ServiceRegistry {
            id_counter: 0,
            services: HashMap::new()
        }
    }

    pub fn get_service_by_name(&self, name: &str) -> Option<&dyn Any> {
        let sl = self.get_services_by_name(name);

        for s in sl {
            return Some(s);
        }
        None
    }

    pub fn get_services_by_name(&self, name: &str) -> Vec<&dyn Any> {
        let mut res = vec![];

        for (p, s) in &self.services {
            let n = p.get("name").expect("'name' property missing");

            if n == name {
                res.push(s.as_ref());
            }
        }

        res
    }

    pub fn get_svc<T: 'static>(&self) -> Option<&T> {
        for (_, s) in &self.services {
            if s.is::<T>() {
                return s.downcast_ref();
            }
        }
        None
    }

    pub fn get_svcs<T: 'static>(&self) -> Vec<&T> {
        let mut res = vec![];

        for (_, s) in &self.services {
            if let Some(dc) = s.downcast_ref() {
                res.push(dc);
            }
        }

        res
    }

    pub fn get_all_svcs(&self) -> Vec<&Box<dyn Any>> {
        let mut res = vec![];

        for (_, s) in &self.services {
            res.push(s);
        }
        res
    }


    /*
    pub fn get_svc<T: 'static>(&self, name: &str) -> Option<&T> {
        let svc = self.get_service_by_name(name);
        match svc {
            Some(s) => {
                return s.downcast_ref();
            },
            None => None
        }
    }

    pub fn get_svcs<T: 'static>(&self, name: &str) -> Vec<&T> {
        let mut res = vec![];

        for s in self.get_services_by_name(name) {
            let dc = s.downcast_ref();
            match dc {
                Some(s) => res.push(s),
                None => ()
            }
        }

        res
    }
     */

    pub fn register_service(&mut self, name: &str, svc: Box<dyn Any>) {
        let mut props: BTreeMap<String, String> = BTreeMap::new();
        props.insert("service.id".to_string(), self.id_counter.to_string());
        props.insert("name".to_string(), name.to_string());
        self.id_counter += 1;

        self.services.insert(props, svc);

        self.status();
    }

    fn status(&self) {
        println!("Known services: {:?}", self.services);
    }
}

#[cfg(test)]
mod tests {
    use super::ServiceRegistry;

    #[test]
    fn test_register_service() {
        let mut sr = ServiceRegistry::new();

        let svc = String::from("My Service");

        assert_eq!(0, sr.get_svcs::<String>().len());
        sr.register_service("mysvc", Box::new(svc));

        let svcs = sr.get_svcs::<String>();
        assert_eq!(1, svcs.len());
        assert_eq!("My Service".to_string(), svcs.get(0).unwrap().to_string());
    }
}