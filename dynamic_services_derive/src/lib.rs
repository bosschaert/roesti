use std::{collections::HashMap, fs};
use std::path::Path;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{self, DataStruct, GenericArgument, PathArguments, Data, DataEnum, DataUnion,
    DeriveInput, Error, Fields, Result, token, Type, Ident};
use proc_macro2::Span;
use serde_json;

#[derive(Debug)]
enum Action {
    CtorInjectField{field: String, type_name: String},
    SetterInjectField{field: String, type_name: String},
    ActivatorFunct{func_name: String}
}

impl Action {
    fn to_string(&self) -> String {
        match self {
            Action::CtorInjectField{field, type_name} => {
                format!("CtorInjectField {} {}", field, type_name)
            },
            Action::SetterInjectField{field, type_name} => {
                format!("{{\"op\":\"SetterInjectField\", \"field\":\"{}\", \"type\":\"{}\"}}", field, type_name)
            },
            Action::ActivatorFunct{func_name} => {
                format!("ActivatorFunct {}", func_name)
            }
        }
    }
}

#[proc_macro_derive(DynamicServices, attributes(inject, constructor))]
pub fn dynamic_services_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // println!("mfdir {:?}", env!("CARGO_MANIFEST_DIR"));
    // println!("mfdir rt {:?}", std::env::var("CARGO_MANIFEST_DIR"));

    // Build the trait implementation
    impl_dynamic_services(ast)
}

fn impl_dynamic_services(ast: syn::DeriveInput) -> TokenStream {
    let (tn, actions) = match find_injected_fields(ast.clone()) {
        Ok(t) => t,
        Err(err) => return TokenStream::from(err.to_compile_error())
    };
    println!("actions: {:?}", actions);

    let name = ast.ident;

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

    let mut gen = quote!{};
    /*
    for (i, s) in &types {
        let ti = format_ident!("{}", s);
        let set_ts = format_ident!("set_{}", s);
        let registry = format_ident!("SERVICES_{}", s);
        let new_code = quote! {
            impl<'_ds> #name<'_ds> {
                pub fn #set_ts(&mut self, svc: &'_ds #ti) {
                    self.#i = Some(svc);
                }

                pub fn new(svc: &'_ds #ti) -> Self {
                    let mut ds = #name {
                        #i: Some(svc),
                        ..Default::default()
                    };

                    ds
                }
            }

            pub static #registry: std::sync::Mutex<Vec<#ti>> = std::sync::Mutex::new(Vec::new());

            // Add the register_service() method to the ServiceRegistry
            impl crate::service_registry::ServiceRegistry {
                pub fn register_service(&mut self, svc: #ti) {
                    println!("Registering service: {:?}", svc);
                    let mut vec = #registry.lock().unwrap();
                    vec.push(svc);

                    let myref = vec.last().unwrap();
                    let cons = #name::new(myref);
                    println!("Created consumer {:?} for {:?}", cons, myref);
                }
            }
        };
        gen.extend(new_code);
    }
    */
    gen.into()
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
        println!("To inject {:?}", f.ident.as_ref().unwrap());

        if let syn::Type::Path(ref_type) = &f.ty {
            let id = f.ident.as_ref().unwrap();
            if let Some(a) = get_type_name(id, ref_type) {
                actions.push(a);
            }
        }
    }

    Ok((ast.ident.to_string(), actions))
}

fn find_attribute(f: &syn::Field, name: &str) -> bool {
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
        println!("*** Try this: {:?}", s);
        if s.ident.to_string() != "Option" {
            println!("*** Not an Option: {:?}", s);
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
            return get_type(ident, t);
        }
    }
    None
}

fn get_type(ident: &syn::Ident, t: &syn::Type) -> Option<Action> {
    if let Type::Reference(r) = t {
        return get_reference(ident, r);
    }
    None
}

fn get_reference(ident: &syn::Ident, r: &syn::TypeReference) -> Option<Action> {
    return if let Type::Path(p) = &*r.elem {
        get_from_typepath(ident, p)
    } else {
        None
    }
}

fn get_from_typepath(ident: &syn::Ident, tp: &syn::TypePath) -> Option<Action> {
    return get_from_pathsegment(ident, &(tp.path).segments);
}

fn get_from_pathsegment(ident: &syn::Ident, segs: &syn::punctuated::Punctuated<syn::PathSegment, token::PathSep>) -> Option<Action> {
    return if let Some(ps) = segs.first() {
        println!("*** Found: {}", ps.ident);
        Some(Action::SetterInjectField { field: ident.to_string(), type_name: ps.ident.to_string() })
    } else {
        None
    }
}

#[proc_macro_attribute]
pub fn activator(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("activator attr: \"{}\"", attr.to_string());
    println!("activator item: \"{}\"", item.to_string());
    item
}

#[proc_macro_attribute]
pub fn dynamic_services(attr: TokenStream, item: TokenStream) -> TokenStream {
    // generate new() constructor with all the required types
    // add the activator() callback too

    // Read from _dynsvc_Consumer2.json
    // required fields
    // new(reqfield1, reqfield2, ...)

    println!("$$xyz attr: \"{}\"", attr.to_string());
    println!("$$xyz item: \"{}\"", item.to_string());
    let toks: Result<syn::ItemImpl> = syn::parse(item.clone().into());
    println!("parsed {:?}", toks);
    let tokens = toks.unwrap();
    let ty = tokens.self_ty;
    let x = if let Type::Path(tp) = ty.as_ref() {
        tp.path.segments.first().unwrap()

        // println!("tp: {:?}", tp);
        // let segs = &tp.path.segments;
        // for s in segs {
        //     println!("seg: {:?}", s);
        // }
    } else {
        panic!("Not a path");
    };
    // println!("ty: {:?}", ty);

    println!("*** ident {}", x.ident);
    let type_name = x.ident.to_string();
    // let type_name = "Foo";

    // let mut generated = quote! {};
    let mut generated: proc_macro2::TokenStream = item.into();

    // list files in target directory starting with _ and ending with .tmp
    let file = format!("{}/target/_{}.tmp", std::env::var("CARGO_MANIFEST_DIR").unwrap(), type_name);
    if Path::new(&file).exists() {
        println!("Generating from {}", file);
        generate_class(&file, &type_name, &mut generated);
    }
    /*
    println!("*** Looking for files in {}", dir);
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        // println!("*** Found file {:?}", path);
        if let Ok(de) = path {
            // let p = de.path();
            // let filenm = p.to_str().unwrap();
            let filenm = de.file_name().into_string().unwrap();
            println!("*** Found file name {}", filenm);
            if filenm.starts_with("_") && filenm.ends_with(".tmp") {
                generate_class(de.path().to_str().unwrap(), &filenm, &mut generated);
            }
        }
    }
     */
    // let mut new_item = item.clone();
    // item.
    // item.extend(generated.into());
    // generated.into()
    // let g = quote! {
    //     impl Consumer1 {
    //         pub fn dummy() {
    //             println!("Dummy");
    //         }
    //     }
    // };
    // let h: proc_macro2::TokenStream = g.into();
    // g.into()
    // item.extend(g.into())
    // new_item.extend(g.into());
    // <proc_macro::TokenStream as Extend<_>>::extend::<_>(&mut new_item, g.into());
    // proc_macro::TokenStream::extend(&mut new_item, h.into());
    // new_item.extend(h.into());

    // new_item2.extend(g);

    // let x = quote! {
    //     impl<'_ds> Consumer1<'_ds> {
    //         pub fn set_TidalService(&mut self, svc: &'_ds TidalService) {
    //             self.tidal = Some(svc);
    //         }
    //     }
    // };
    // generated.extend(x);

    generated.into()
    // quote! {}.into()
}

fn generate_class(file_path: &str, type_name: &str, generated: &mut proc_macro2::TokenStream) {
    println!("*** Generate class from file {} for type name {}", file_path, type_name);
    let content = fs::read_to_string(file_path).expect("Unable to read file");
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();

    for action in json.as_array().unwrap() {
        generated.extend(generate_action(type_name, action));
    }
}

fn generate_action(type_name: &str, action: &serde_json::Value) -> proc_macro2::TokenStream {
    let op = action["op"].as_str().unwrap();
    match op {
        "SetterInjectField" => {
            let field = action["field"].as_str().unwrap();
            let injected_type_name = action["type"].as_str().unwrap();

            println!("[{}] SetterInjectField {} {}", type_name, field, injected_type_name);
            let tn = format_ident!("{}", type_name);
            let set_ts = format_ident!("set_{}", injected_type_name);
            let itn = format_ident!("{}", injected_type_name);
            let injected = format_ident!("{}", field);
            let new_code = quote! {
                impl<'_ds> #tn<'_ds> {
                    pub fn #set_ts(&mut self, svc: &'_ds #itn) {
                        println!("Setting {} to {:?}", #field, svc);
                        self.#injected = Some(svc);
                    }
                }
            };
            return new_code;
        },
        _ => {
            panic!("Unknown action: {}", op);
        }
    }
}

#[proc_macro_attribute]
pub fn blah(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("blah attr: \"{}\"", attr.to_string());
    println!("blah item: \"{}\"", item.to_string());
    item
}
