use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::fs;
use std::path::{Path, PathBuf};

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{self, token, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Error, Fields, GenericArgument, ItemFn, PathArguments, Result, Type};
use proc_macro2::Span;
use serde_json;

#[derive(Debug)]
enum Action {
    LifeTimes{names: Vec<String>},
    SetterInjectField{field: String, type_name: String},
    ActivatorFunct{func_name: String},
    DeactivatorFunct{func_name: String}
}

impl Action {
    fn to_string(&self) -> String {
        match self {
            Action::LifeTimes {names} => {
                format!("{{\"op\":\"LifeTimes\", \"names\":{:?} }}", names)
            },
            Action::SetterInjectField{field, type_name} => {
                format!("{{\"op\":\"SetterInjectField\", \"field\":\"{}\", \"type\":\"{}\"}}", field, type_name)
            },
            Action::ActivatorFunct{func_name} => {
                format!("{{\"op\":\"ActivatorFunct\", \"method\":\"{}\"}}", func_name)
            },
            Action::DeactivatorFunct{func_name} => {
                format!("{{\"op\":\"DeactivatorFunct\", \"method\":\"{}\"}}", func_name)
            }
        }
    }
}

#[proc_macro_derive(DynamicServices, attributes(inject, constructor))]
pub fn dynamic_services_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_dynamic_services(ast)
}

fn impl_dynamic_services(ast: syn::DeriveInput) -> TokenStream {
    let (tn, actions) = match find_injected_fields(ast.clone()) {
        Ok(t) => t,
        Err(err) => return TokenStream::from(err.to_compile_error())
    };

    let mut lines = vec![];
    lines.push("[".to_string());
    let mut first = true;
    for a in &actions {
        if first {
            first = false;
        } else {
            lines.push(",".to_string());
        }

        lines.push(a.to_string());
    }
    lines.push("]".to_string());
    write_actions_file(tn, lines);

    quote!{}.into()
}

fn write_actions_file(tn: String, lines: Vec<String>) {
    if lines.len() == 0 {
        return;
    }

    let filenm = format!("{}/target/_{}.tmp", std::env::var("CARGO_MANIFEST_DIR").unwrap(), tn);

    let mut content = lines.join("\n");
    content.push('\n');
    std::fs::write(filenm, content).expect("Unable to write file");
}

fn find_injected_fields(ast: DeriveInput)
  -> Result<(String, Vec<Action>)> {
    let fields = match ast.data {
        | Data::Enum(DataEnum { enum_token: token::Enum { span }, ..})
        | Data::Union(DataUnion { union_token: token::Union { span }, ..})
        => {
            return Err(Error::new(span, "expected a struct"));
        },
        | Data::Struct(DataStruct { fields: Fields::Named(it), .. })
        => {
            it
        },
        | Data::Struct(_)
        => {
            return Err(Error::new(Span::call_site(), "expected a struct with named fields"));
        },
    };

    let mut actions = Vec::new();
    for f in fields.named.iter() {
        if !find_attribute(f, "inject") {
            continue;
        }

        if let syn::Type::Path(ref_type) = &f.ty {
            let id = f.ident.as_ref().unwrap();
            if let Some(a) = get_type_name(id, ref_type) {
                actions.push(a);
            }
        }
    }

    actions.push(get_lifetimes(&ast.generics.params));

    Ok((ast.ident.to_string(), actions))
}

fn get_lifetimes(params: &syn::punctuated::Punctuated<syn::GenericParam, token::Comma>) -> Action {
    let mut lifetimes = vec![];

    for param in params.iter() {
        match param {
            | syn::GenericParam::Lifetime(lt)
            => {
                lifetimes.push(lt.lifetime.ident.to_string());
            },
            | _
            => {}
        }
    }

    Action::LifeTimes { names: lifetimes }
}


fn find_attribute(f: &syn::Field, _name: &str) -> bool {
    for a in &f.attrs {
        if let Some(name) = a.path().get_ident() {
            if name == "inject" {
                return true;
            }
        }
    }
    false
}

fn get_type_name(ident: &syn::Ident, ref_type: &syn::TypePath) -> Option<Action> {
    for s in ref_type.path.segments.iter() {
        if s.ident.to_string() != "Option" {
            return None;
        }

        return match &s.arguments {
            | PathArguments::AngleBracketed(aba)
            => get_option_args(ident, aba),
            | _ => None
        };
    }
    None
}

fn get_option_args(ident: &syn::Ident, aba: &syn::AngleBracketedGenericArguments) -> Option<Action> {
    for a in aba.args.iter() {
        if let GenericArgument::Type(t) = a {
            return get_servicereference_type(ident, t);
        }
    }
    None
}

fn get_servicereference_type(ident: &syn::Ident, t: &syn::Type) -> Option<Action> {
    if let Type::Path(p) = t {
        return get_from_typepath(ident, p);
    }
    None
}

fn get_from_typepath(ident: &syn::Ident, tp: &syn::TypePath) -> Option<Action> {
    return get_from_pathsegment(ident, &(tp.path).segments);
}

fn get_from_pathsegment(ident: &syn::Ident, segs: &syn::punctuated::Punctuated<syn::PathSegment, token::PathSep>) -> Option<Action> {
    if let Some(ps) = segs.first() {
        if ps.ident.to_string() == "ServiceReference" {
            return get_from_serviceref(ident, &ps.arguments);
        }
    }
    None
}

fn get_from_serviceref(ident: &syn::Ident, arguments: &PathArguments) -> Option<Action> {
    return match &arguments {
        | PathArguments::AngleBracketed(aba)
        => get_serviceref_typearg(ident, aba),
        | _ => None
    };
}

fn get_serviceref_typearg(ident: &syn::Ident, aba: &syn::AngleBracketedGenericArguments) -> Option<Action> {
    if let Some(arg) = aba.args.first() {
        if let GenericArgument::Type(t) = arg {
            if let Type::Path(tp) = t {
                if let Some(tn) = tp.path.segments.first() {
                    return Some(Action::SetterInjectField { field: ident.to_string(), type_name: tn.ident.to_string() });
                }
            }
        }
    }
    None
}


// TODO can this be done as part of the dynamic_services attribute macro as its embedded in there
#[proc_macro_attribute]
pub fn activator(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse(item.clone()).unwrap();
    write_activator_fn(ast);

    item
}

fn write_activator_fn(ast: ItemFn) {
    let cur_type = CUR_TYPE.lock().unwrap();

    let filenm = format!("{}/target/_{}.acttmp", std::env::var("CARGO_MANIFEST_DIR").unwrap(), cur_type);
    let act = Action::ActivatorFunct{func_name: ast.sig.ident.to_string()};
    let content = format!("[{}]", act.to_string());
    std::fs::write(filenm, content).unwrap();
}

// TODO can this be done as part of the dynamic_services attribute macro as its embedded in there
#[proc_macro_attribute]
pub fn deactivator(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse(item.clone()).unwrap();
    write_deactivator_fn(ast);

    item
}

fn write_deactivator_fn(ast: ItemFn) {
    let cur_type = CUR_TYPE.lock().unwrap();

    let filenm = format!("{}/target/_{}.deacttmp", std::env::var("CARGO_MANIFEST_DIR").unwrap(), cur_type);
    let act = Action::DeactivatorFunct{func_name: ast.sig.ident.to_string()};
    let content = format!("[{}]", act.to_string());
    std::fs::write(filenm, content).unwrap();
}

static CUR_TYPE: Lazy<Mutex<String>> = Lazy::new(||Mutex::new(String::new()));

// For impl classes
#[proc_macro_attribute]
pub fn dynamic_services(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let toks: Result<syn::ItemImpl> = syn::parse(item.clone().into());
    let tokens = toks.unwrap();
    let ty = tokens.self_ty;
    let x = if let Type::Path(tp) = ty.as_ref() {
        tp.path.segments.first().unwrap()
    } else {
        panic!("Not a path");
    };

    let type_name = x.ident.to_string();

    // set current type to type_name;
    *CUR_TYPE.lock().unwrap() = type_name.clone();

    let mut generated: proc_macro2::TokenStream = item.into();

    let file = format!("{}/target/_{}.tmp", std::env::var("CARGO_MANIFEST_DIR").unwrap(), type_name);
    if Path::new(&file).exists() {
        generate_class(&file, &type_name, &mut generated);
    }

    generated.into()
}

fn generate_class(file_path: &str, type_name: &str, generated: &mut proc_macro2::TokenStream) {
    let content = fs::read_to_string(file_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();

    let lifetimes = get_lifetimes_from_json(json.as_array().unwrap());
    for action in json.as_array().unwrap() {
        generated.extend(generate_action(type_name, action, &lifetimes));
    }
}

fn get_lifetimes_from_json(actions: &[serde_json::Value]) -> Vec<String> {
    let mut lifetimes = vec![];

    for action in actions {
        let op = action["op"].as_str().unwrap();
        if op == "LifeTimes" {
            if let Some(names) = action["names"].as_array() {
                names.iter().for_each(|v| {
                    if let Some(lt) = v.as_str() {
                        lifetimes.push(lt.to_string());
                    }
                });
            }
        }
    }
    lifetimes
}

fn generate_action(type_name: &str, action: &serde_json::Value, lifetimes: &Vec<String>) -> proc_macro2::TokenStream {
    let lifetimes_code = quote_fixed_lifetimes(lifetimes.len(), quote! { '_ });

    let op = action["op"].as_str().unwrap();
    match op {
        "SetterInjectField" => {
            let field = action["field"].as_str().unwrap();
            let injected_type_name = action["type"].as_str().unwrap();

            let tn = format_ident!("{}", type_name);
            let set_ts_ref = format_ident!("set_{}_ref", injected_type_name);
            let itn = format_ident!("{}", injected_type_name);
            let injected_ref = format_ident!("{}", field);
            let invoke_svc = format_ident!("invoke_{}", field);
            let new_code = quote! {
                impl #tn #lifetimes_code {
                    pub fn #set_ts_ref(&mut self, sreg: &ServiceRegistration) {
                        println!("[{}] Setting {} to {:?}", #type_name, #field, sreg);
                        self.#injected_ref = Some(ServiceReference::from(sreg));
                    }

                    pub fn unset_all(&mut self) {
                        println!("[{}] Unsetting all injected fields", #type_name);
                        self.#injected_ref = None;
                    }

                    fn #invoke_svc(&self, cb: impl Fn (&#itn)) {
                        let sr = REGD_SERVICES.read().unwrap();
                        let sref = &self.#injected_ref.as_ref().unwrap();
                        let sreg = ServiceRegistration::from(sref);
                        let svc = sr.get(&sreg).unwrap();
                        if let Some(sr) = svc.downcast_ref::<#itn>() {
                            cb(sr);
                        }
                    }
                }
            };
            return new_code;
        },
        "LifeTimes" => {
            return quote!{};
        },
        _ => {
            panic!("Unknown action: {}", op);
        }
    }
}

// For the main class
#[proc_macro_attribute]
pub fn dynamic_services_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut generated: proc_macro2::TokenStream = item.into();

    let new_code = quote! {
        fn register_service(svc: Box<dyn Any + Send + Sync>) -> ServiceRegistration {
            register_consumers();

            let sreg = ServiceRegistration::new();
            println!("Registering service: {:?} - {:?}", svc, sreg);
            REGD_SERVICES.write().unwrap().insert(sreg, svc);

            inject_consumers();
            sreg
        }

        fn unregister_service(sr: ServiceRegistration) {
            if REGD_SERVICES.write().unwrap().remove(&sr).is_some() {
                println!("Service unregistered: {:?}", sr);
                uninject_consumers(&sr);
            }
        }
    };
    generated.extend(new_code);

    let mut consumer_types = vec![];
    let dir = format!("{}/target", std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        if let Ok(p) = path {
            if let Some((name, tokens)) = generate_consumer(p.path(), p.file_name().to_str().unwrap()) {
                consumer_types.push(name);
                generated.extend(tokens);
            }
        }
    }

    generated.extend(generate_register_consumers(&consumer_types));
    generated.extend(generate_inject_consumers(&consumer_types));
    generated.extend(generate_uninject_consumers(&consumer_types));

    generated.into()
}

fn quote_fixed_lifetimes(num: usize, lt: proc_macro2::TokenStream ) -> proc_macro2::TokenStream {
    let lifetimes = vec![lt; num];
    quote! { <#(#lifetimes),*> }
}

fn generate_consumer(path: PathBuf, file_name: &str) -> Option<(String, proc_macro2::TokenStream)> {
    if file_name.starts_with("_") && file_name.ends_with(".tmp") {
        let content = fs::read_to_string(path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        let lifetimes = get_lifetimes_from_json(json.as_array().unwrap());
        let static_lifetimes = quote_fixed_lifetimes(lifetimes.len(), quote! { 'static });

        let type_name = &file_name[1..file_name.len()-4];
        let tn = format_ident!("{}", type_name);
        let register_fn = format_ident!("register_{}", type_name);
        let global_ctor_map = format_ident!("CONSUMER_CTOR_{}", type_name.to_uppercase());
        let global_inst_map = format_ident!("CONSUMER_INST_{}", type_name.to_uppercase());

        let inject_function = generate_inject_function(json, type_name);

        let tokens = quote!{
            static #global_ctor_map: Lazy<Mutex<Vec<fn() -> #tn #static_lifetimes>>>
                = Lazy::new(||Mutex::new(Vec::new()));
            static #global_inst_map: Lazy<Mutex<HashMap<ConsumerRegistration, (#tn, Vec<ServiceRegistration>)>>>
                = Lazy::new(||Mutex::new(HashMap::new()));

            fn #register_fn() {
                println!("Registering Consumer: {}", #type_name);
                #global_ctor_map.lock().unwrap().push(|| #tn::default());
            }

            #(#inject_function)*
        };
        return Some((type_name.to_string(), tokens));
    }
    None
}

fn generate_inject_function(json: serde_json::Value, type_name: &str) -> Vec<proc_macro2::TokenStream> {
    let mut quotes = vec![];

    let act_call = generate_activator_call(type_name);
    let deact_call = generate_deactivator_call(type_name);

    for action in json.as_array().unwrap() {
        let op = action["op"].as_str().unwrap();
        match op {
            "SetterInjectField" => {
                let injected_type_name = action["type"].as_str().unwrap();
                let inject_fn = format_ident!("inject_{}", type_name);
                let uninject_fn = format_ident!("uninject_{}", type_name);
                let itn = format_ident!("{}", injected_type_name);
                let global_ctor_map = format_ident!("CONSUMER_CTOR_{}", type_name.to_uppercase());
                let global_inst_map = format_ident!("CONSUMER_INST_{}", type_name.to_uppercase());
                let setter_ref = format_ident!("set_{}_ref", injected_type_name);
                let q = quote! {
                    fn #inject_fn(svc: &Box<dyn Any + Send + Sync>, sreg: &ServiceRegistration) {
                        if let Some(sr) = svc.downcast_ref::<#itn>() {
                            for ctor in #global_ctor_map.lock().unwrap().iter() {
                                let mut c = ctor();
                                c.#setter_ref(sreg);

                                #act_call(sr);

                                // Keep the consumer instance in the global map
                                let regs = vec![sreg.clone()];
                                #global_inst_map.lock().unwrap().insert(
                                    ConsumerRegistration::new(),
                                    (c, regs));
                            }
                        }
                    }

                    fn #uninject_fn(sreg: &ServiceRegistration) {
                        let mut deleted = vec![];
                        let mut global = #global_inst_map.lock().unwrap();
                        global.iter_mut()
                            .filter(|(_, (_, regs))| regs.contains(sreg))
                            .for_each(|(ci, (c, _))| {
                                deleted.push(ci.clone());
                                c.unset_all();
                                #deact_call
                            });
                        deleted.iter().for_each(|ci| { global.remove(ci); });
                    }
                };
                quotes.push(q);
            },
            "LifeTimes" => {
                // ignore
            },
            _ => {
                panic!("Unknown action: {}", op);
            }
        }
    }
    quotes
}

fn generate_activator_call(type_name: &str) -> proc_macro2::TokenStream {
    let mut new_code = quote! {};

    let file = format!("{}/target/_{}.acttmp", std::env::var("CARGO_MANIFEST_DIR").unwrap(), type_name);
    if Path::new(&file).exists() {
        generate_activator(&file, &mut new_code);
    }

    new_code
}

fn generate_activator(file: &str, new_code: &mut proc_macro2::TokenStream) {
    let acttmp_content = fs::read_to_string(file).unwrap();
    let json: serde_json::Value = serde_json::from_str(&acttmp_content).unwrap();

    for action in json.as_array().unwrap() {
        let op = action["op"].as_str().unwrap();
        match op {
            "ActivatorFunct" => {
                let func_name = action["method"].as_str().unwrap();
                let activate_md = format_ident!("{}", func_name);
                new_code.extend(quote! {
                    c.#activate_md
                });
            },
            _ => {
                panic!("Unknown action: {}", op);
            }
        }
    }
}

fn generate_deactivator_call(type_name: &str) -> proc_macro2::TokenStream {
    let mut new_code = quote! {};

    let file = format!("{}/target/_{}.deacttmp", std::env::var("CARGO_MANIFEST_DIR").unwrap(), type_name);
    if Path::new(&file).exists() {
        generate_deactivator(&file, &mut new_code);
    }

    new_code
}

// TODO collapse with activator
fn generate_deactivator(file: &str, new_code: &mut proc_macro2::TokenStream) {
    let acttmp_content = fs::read_to_string(file).unwrap();
    let json: serde_json::Value = serde_json::from_str(&acttmp_content).unwrap();

    for action in json.as_array().unwrap() {
        let op = action["op"].as_str().unwrap();
        match op {
            "DeactivatorFunct" => {
                let func_name = action["method"].as_str().unwrap();
                let activate_md = format_ident!("{}", func_name);
                new_code.extend(quote! {
                    c.#activate_md();
                });
            },
            _ => {
                panic!("Unknown action: {}", op);
            }
        }
    }
}

fn generate_register_consumers(consumer_types: &Vec<String>) -> proc_macro2::TokenStream {
    let mut register_calls = vec![];
    for ct in consumer_types {
        let register_fn = format_ident!("register_{}", ct);
        register_calls.push(quote!{
            #register_fn();
        });
    }

    let new_code = quote! {
        static CONSUMERS_INITIALIZED: AtomicBool = AtomicBool::new(false);
        fn register_consumers() {
            let initialized = CONSUMERS_INITIALIZED.swap(true, Ordering::SeqCst);
            if initialized {
                return;
            }

            #(#register_calls)*
        }
    };
    new_code
}

fn generate_inject_consumers(consumer_types: &Vec<String>) -> proc_macro2::TokenStream {
    let mut inject_calls = vec![];
    for ct in consumer_types {
        let inject_fn = format_ident!("inject_{}", ct);
        inject_calls.push(quote!{
            #inject_fn(svc, &sreg);
        });
    }

    let new_code = quote! {
        // TODO only inject the relevant consumers and don't re-inject
        fn inject_consumers() {
            for (sreg, svc) in REGD_SERVICES.read().unwrap().iter() {
                #(#inject_calls)*
            }
        }
    };
    new_code
}

fn generate_uninject_consumers(consumer_types: &Vec<String>) -> proc_macro2::TokenStream {
    // All consumers have in their global map as a value the list in dependent service
    // references. Un-inject all consumers that have the service reference of the service
    // being unregistered.

    let mut uninject_calls = vec![];
    for ct in consumer_types {
        // let global_inst_map = format_ident!("CONSUMER_INST_{}", type_name.to_uppercase());
        let uninject_fn = format_ident!("uninject_{}", ct);
        uninject_calls.push(quote!{
            #uninject_fn(sr);
        });
    }

    quote! {
        fn uninject_consumers(sr: &ServiceRegistration) {
            #(#uninject_calls)*
        }
    }
}

