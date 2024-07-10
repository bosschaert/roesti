use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::Parser;
use syn::{self, token, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Error, Fields, GenericArgument, PathArguments, Result, Type};
use proc_macro2::Span;
use serde_json;

#[derive(Debug)]
enum Action {
    LifeTimes{names: Vec<String>},
    SetterInjectField{field: String, type_name: String},
    ActivatorFunct{func_name: String, arguments: Vec<String>},
    DeactivatorFunct{func_name: String},
    StructPath{path: String}
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
            Action::ActivatorFunct{func_name, arguments} => {
                format!("{{\"op\":\"ActivatorFunct\", \"method\":\"{}\", \"args\":{:?} }}", func_name, arguments)
            },
            Action::DeactivatorFunct{func_name} => {
                format!("{{\"op\":\"DeactivatorFunct\", \"method\":\"{}\"}}", func_name)
            }
            Action::StructPath { path } => {
                format!("{{\"op\":\"StructPath\", \"path\":\"{}\"}}", path)
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

#[proc_macro_attribute]
pub fn activator(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Nothing to do, as this is handled by the #dynamic_services macro
    item
}

#[proc_macro_attribute]
pub fn deactivator(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Nothing to do, as this is handled by the #dynamic_services macro
    item
}

// For impl classes
#[proc_macro_attribute]
pub fn dynamic_services(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let toks: Result<syn::ItemImpl> = syn::parse(item.clone().into());
    let tokens = toks.unwrap();

    let impl_type_box = &tokens.self_ty;
    let impl_type = if let Type::Path(tp) = impl_type_box.as_ref() {
        tp.path.segments.first().unwrap()
    } else {
        panic!("Not a path");
    };

    let type_name = impl_type.ident.to_string();

    if let Some(activator) = find_activator(&tokens) {
        write_action(activator, &type_name, "acttmp");
    }
    if let Some(deactivator) = find_deactivator(&tokens) {
        write_action(deactivator, &type_name, "deacttmp");
    }
    if let Some(path) = get_struct_path(attrs) {
        write_action(path, &type_name, "pathtmp");
    }

    let mut generated: proc_macro2::TokenStream = item.into();

    let file = format!("{}/target/_{}.tmp", std::env::var("CARGO_MANIFEST_DIR").unwrap(), type_name);
    if Path::new(&file).exists() {
        generate_class(&file, &type_name, &mut generated);
    }

    generated.into()
}

fn get_struct_path(attrs: TokenStream) -> Option<Action> {
    let args_parsed = syn::punctuated::Punctuated::<syn::ExprAssign, syn::Token![,]>::parse_terminated.parse(attrs).unwrap();
    for arg in args_parsed.iter() {
        if let syn::Expr::Path(key) = arg.left.as_ref() {
            if let Some(ps) = key.path.segments.first() {
                if ps.ident.to_string() == "path" {
                    if let syn::Expr::Path(p) = arg.right.as_ref() {
                        return get_full_path(p);
                    }
                }
            }
        }
    }
    None
}

fn get_full_path(path: &syn::ExprPath) -> Option<Action> {
    let mut strings = vec![];

    for el in path.path.segments.iter() {
        strings.push(el.ident.to_string());
    }
    if !strings.is_empty() {
        return Some(Action::StructPath { path: strings.join("::") });
    }
    None
}

fn write_action(action: Action, curtype: &str, suffix: &str) {
    let filenm = format!("{}/target/_{}.{}", std::env::var("CARGO_MANIFEST_DIR").unwrap(), curtype, suffix);
    let content = format!("[{}]", action.to_string());
    std::fs::write(filenm, content).unwrap();
}

fn find_lifecycle_callback(ls: &str, itimpl: &syn::ItemImpl) -> Option<(String, Vec<String>)> {
    for item in itimpl.items.iter() {
        if let syn::ImplItem::Fn(f) = item {
            for a in f.attrs.iter() {
                if let Some(an) = a.meta.path().get_ident() {
                    if an.to_string() == ls {
                        let inputs = get_inputs_from_fn(&f.sig.inputs);
                        return Some((f.sig.ident.to_string(), inputs));
                    }
                }
            }
        }
    };

    None
}

fn find_activator(itimpl: &syn::ItemImpl) -> Option<Action> {
    let act = find_lifecycle_callback("activator", itimpl);
    if let Some((name, args)) = act {
        return Some(Action::ActivatorFunct { func_name: name, arguments: args });
    }
    None
}

fn find_deactivator(itimpl: &syn::ItemImpl) -> Option<Action> {
    let deact = find_lifecycle_callback("deactivator", itimpl);
    if let Some((name, _)) = deact {
        return Some(Action::DeactivatorFunct { func_name: name });
    }
    return None;
}

fn get_inputs_from_fn(inputs: &syn::punctuated::Punctuated<syn::FnArg, token::Comma>) -> Vec<String> {
    let mut counter = 0;
    let mut args = vec![];

    for input in inputs {
        match input {
            | syn::FnArg::Receiver(_r)
            => {
                if counter > 0 {
                    panic!("Only the first argument should be a Self reference");
                }
            },
            | syn::FnArg::Typed(arg)
            => {
                if counter == 0 {
                    panic!("The first argument should be a Self reference");
                }
                if let syn::Type::Reference(tr) = arg.ty.as_ref() {
                    if let syn::Type::Path(tp) = tr.elem.as_ref() {
                        if let Some(tn) = tp.path.segments.first() {
                            args.push(format!("&{}", tn.ident.to_string()));
                        }
                    }
                }
            }
        }
        counter += 1;
    }

    args
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
                    pub fn #set_ts_ref(&mut self, sreg: &crate::service_registry::ServiceRegistration) {
                        println!("[{}] Setting {} to {:?}", #type_name, #field, sreg);
                        self.#injected_ref = Some(ServiceReference::from(sreg));
                    }

                    pub fn unset_all(&mut self) {
                        println!("[{}] Unsetting all injected fields", #type_name);
                        self.#injected_ref = None;
                    }

                    fn #invoke_svc(&self, cb: impl Fn (&#itn)) {
                        let sr = crate::service_registry::REGD_SERVICES.read().unwrap();
                        let sref = &self.#injected_ref.as_ref().unwrap();
                        let sreg = crate::service_registry::ServiceRegistration::from(sref);
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
        // TODO support service properties
        fn register_service(svc: Box<dyn ::std::any::Any + Send + Sync>) -> ::roesti::service_registry::ServiceRegistration {
            register_consumers();

            let sreg = ::roesti::service_registry::ServiceRegistration::new();
            println!("Registering service: {:?} - {:?}", svc, sreg);
            ::roesti::service_registry::REGD_SERVICES.write().unwrap().insert(sreg, svc);

            inject_consumers();
            sreg
        }

        fn unregister_service(sr: ::roesti::service_registry::ServiceRegistration) {
            if ::roesti::service_registry::REGD_SERVICES.write().unwrap().remove(&sr).is_some() {
                println!("Service unregistered: {:?}", sr);
                uninject_consumers(&sr);
            }
        }
    };
    generated.extend(new_code);

    let mut consumer_types = HashMap::new();
    let dir = format!("{}/target", std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        if let Ok(p) = path {
            if let Some((name, path, tokens)) = generate_consumer(p.path(), p.file_name().to_str().unwrap()) {
                consumer_types.insert(name, path);
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

fn generate_consumer(path: PathBuf, file_name: &str) -> Option<(String, String, proc_macro2::TokenStream)> {
    if file_name.starts_with("_") && file_name.ends_with(".tmp") {
        let content = fs::read_to_string(&path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        let lifetimes = get_lifetimes_from_json(json.as_array().unwrap());
        let static_lifetimes = quote_fixed_lifetimes(lifetimes.len(), quote! { 'static });

        let type_name = &file_name[1..file_name.len()-4];

        let mut path_file = path.clone();
        path_file.pop();
        path_file.push(format!("_{}.pathtmp", type_name));
        let path = if path_file.exists() {
            let content = fs::read_to_string(path_file).unwrap();
            let json: serde_json::Value = serde_json::from_str(&content).unwrap();
            if let Some(p) = get_path_from_json(json.as_array().unwrap()) {
                p
            } else {
                type_name.to_string()
            }
        } else {
            type_name.to_string()
        };

        // let tn = format_ident!("{}::{}", path, type_name);
        let tn = format_ident!("{}", type_name);
        let register_fn = format_ident!("register_{}", type_name);
        let global_ctor_map = format_ident!("CONSUMER_CTOR_{}", type_name.to_uppercase());
        let global_inst_map = format_ident!("CONSUMER_INST_{}", type_name.to_uppercase());

        let inject_function = generate_inject_function(json, type_name);

        let tokens = quote!{
            static #global_ctor_map: ::once_cell::sync::Lazy<std::sync::Mutex<Vec<fn() -> #tn #static_lifetimes>>>
                = ::once_cell::sync::Lazy::new(||std::sync::Mutex::new(Vec::new()));
            static #global_inst_map: ::once_cell::sync::Lazy<std::sync::Mutex<
                    std::collections::HashMap<::roesti::service_registry::ConsumerRegistration,
                        (#tn, Vec<::roesti::service_registry::ServiceRegistration>)>>>
                = ::once_cell::sync::Lazy::new(||std::sync::Mutex::new(std::collections::HashMap::new()));

            fn #register_fn() {
                println!("Registering Consumer: {}", #type_name);
                #global_ctor_map.lock().unwrap().push(|| #tn::default());
            }

            #(#inject_function)*
        };
        return Some((type_name.to_string(), path, tokens));
    }
    None
}

fn get_path_from_json(actions: &[serde_json::Value]) -> Option<String> {
    for action in actions {
        let op = action["op"].as_str().unwrap();
        if op == "StructPath" {
            return Some(action["path"].as_str().unwrap().to_string());
        }
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
                    fn #inject_fn(svc: &Box<dyn ::std::any::Any + Send + Sync>, sreg: &::roesti::service_registry::ServiceRegistration) {
                        if let Some(sr) = svc.downcast_ref::<#itn>() {
                            for ctor in #global_ctor_map.lock().unwrap().iter() {
                                let mut c = ctor();
                                c.#setter_ref(sreg);

                                #act_call(sr);

                                // Keep the consumer instance in the global map
                                let regs = vec![sreg.clone()];
                                #global_inst_map.lock().unwrap().insert(
                                    ::roesti::service_registry::ConsumerRegistration::new(), (c, regs));
                            }
                        }
                    }

                    fn #uninject_fn(sreg: &::roesti::service_registry::ServiceRegistration) {
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

fn generate_register_consumers(consumer_types: &HashMap<String, String>) -> proc_macro2::TokenStream {
    let mut register_calls = vec![];
    for (ct, _) in consumer_types {
        let register_fn = format_ident!("register_{}", ct);
        register_calls.push(quote!{
            #register_fn();
        });
    }

    let new_code = quote! {
        static CONSUMERS_INITIALIZED: ::std::sync::atomic::AtomicBool =
            ::std::sync::atomic::AtomicBool::new(false);
        fn register_consumers() {
            let initialized = CONSUMERS_INITIALIZED.swap(true, ::std::sync::atomic::Ordering::SeqCst);
            if initialized {
                return;
            }

            #(#register_calls)*
        }
    };
    new_code
}

fn generate_inject_consumers(consumer_types: &HashMap<String, String>) -> proc_macro2::TokenStream {
    let mut inject_calls = vec![];
    for (ct, _) in consumer_types {
        let inject_fn = format_ident!("inject_{}", ct);
        inject_calls.push(quote!{
            #inject_fn(svc, &sreg);
        });
    }

    let new_code = quote! {
        // TODO only inject the relevant consumers and don't re-inject
        fn inject_consumers() {
            for (sreg, svc) in ::roesti::service_registry::REGD_SERVICES.read().unwrap().iter() {
                #(#inject_calls)*
            }
        }
    };
    new_code
}

fn generate_uninject_consumers(consumer_types: &HashMap<String, String>) -> proc_macro2::TokenStream {
    // All consumers have in their global map as a value the list in dependent service
    // references. Un-inject all consumers that have the service reference of the service
    // being unregistered.

    let mut uninject_calls = vec![];
    for (ct, _) in consumer_types {
        // let global_inst_map = format_ident!("CONSUMER_INST_{}", type_name.to_uppercase());
        let uninject_fn = format_ident!("uninject_{}", ct);
        uninject_calls.push(quote!{
            #uninject_fn(sr);
        });
    }

    quote! {
        fn uninject_consumers(sr: &::roesti::service_registry::ServiceRegistration) {
            #(#uninject_calls)*
        }
    }
}

