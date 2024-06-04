use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{self, DataStruct, GenericArgument, PathArguments, Data, DataEnum, DataUnion,
    DeriveInput, Error, Fields, Result, token, Type, Ident};
use proc_macro2::Span;

#[proc_macro_derive(DynamicServices, attributes(inject, constructor))]
pub fn dynamic_services_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_dynamic_services(ast)
}

fn impl_dynamic_services(ast: syn::DeriveInput) -> TokenStream {
    let types = match find_injected_fields(ast.clone()) {
        Ok(t) => t,
        Err(err) => return TokenStream::from(err.to_compile_error())
    };
    println!("Generate for types: {:?}", types);

    let name = ast.ident;
    let mut gen = quote!{};

    for (i, s) in &types {
        let ti = format_ident!("{}", s);
        let set_ts = format_ident!("set_{}", s);
        let new_code = quote! {
            impl<'_ds> #name<'_ds> {
                pub fn #set_ts(&mut self, svc: &'_ds #ti) {
                    self.#i = Some(svc);
                }
            }
        };
        gen.extend(new_code);
    }
    gen.into()
}

fn find_injected_fields(ast: DeriveInput)
  -> Result<HashMap<Ident, String>> {
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

    let mut injected = HashMap::new();
    for f in fields.named.iter() {
        if let syn::Type::Path(ref_type) = &f.ty {
            if let Some(n) = get_type_name(ref_type) {
                injected.insert(f.ident.clone().unwrap(), n);
            }
        }
    }

    Ok(injected)
}

fn get_type_name(ref_type: &syn::TypePath) -> Option<String> {
    for s in ref_type.path.segments.iter() {
        if s.ident.to_string() != "Option" {
            return None;
        }

        return match &s.arguments {
            PathArguments::AngleBracketed(aba) => get_option_args(aba),
            _ => None
        };
    }
    None
}

fn get_option_args(aba: &syn::AngleBracketedGenericArguments) -> Option<String> {
    for a in aba.args.iter() {
        if let GenericArgument::Type(t) = a {
            return get_type(t);
        }
    }
    None
}

fn get_type(t: &syn::Type) -> Option<String> {
    if let Type::Reference(r) = t {
        return get_reference(r);
    }
    None
}

fn get_reference(r: &syn::TypeReference) -> Option<String> {
    return if let Type::Path(p) = &*r.elem {
        get_from_typepath(p)
    } else {
        None
    }
}

fn get_from_typepath(tp: &syn::TypePath) -> Option<String> {
    return get_from_pathsegment(&(tp.path).segments);
}

fn get_from_pathsegment(segs: &syn::punctuated::Punctuated<syn::PathSegment, token::PathSep>) -> Option<String> {
    return if let Some(ps) = segs.first() {
        println!("*** Found: {}", ps.ident);
        Some(ps.ident.to_string())
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
    println!("xyz attr: \"{}\"", attr.to_string());
    println!("xyz item: \"{}\"", item.to_string());
    item
}