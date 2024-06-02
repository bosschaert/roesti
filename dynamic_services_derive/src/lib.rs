use proc_macro::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{self, DataStruct, GenericArgument, LitStr, PathArguments};
use syn::{Data, DataEnum, DataUnion, DeriveInput, Error, Fields, Result, token, Type, TypePath};
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro_derive(DynamicServices, attributes(inject))]
pub fn dynamic_services_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_dynamic_services(ast)
}

fn impl_dynamic_services(ast: syn::DeriveInput) -> TokenStream {
    let types = match find_injected_types(ast.clone()) {
        Ok(t) => t,
        Err(err) => return TokenStream::from(err.to_compile_error())
    };
    println!("Generate for types: {:?}", types);

    let name = ast.ident;
    let ot = types.get(0);
    if ot.is_none() {
        return quote! {}.into();
    }

    let t = ot.unwrap();
    let ti = format_ident!("{}", t);
    let gen = quote! {
        impl<'a> #name<'a> {
            pub fn blah(&self) {
                println!("blah");
            }

            pub fn set_t(&self, xsvc: &'a #ti) {
                println!("Setting {}", #t);
            }
        }
    };
    gen.into()
}


fn find_injected_types(ast: DeriveInput)
  -> Result<Vec<String>> {
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

    let mut names = vec![];
    for f in fields.named.iter() {
        if let syn::Type::Path(ref_type) = &f.ty {
            if let Some(n) = get_type_name(ref_type) {
                names.push(n);
            }
        }
    }

    Ok(names)
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
        println!("a: {:?}", a.to_token_stream());
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


    /*
    for f in fields.named.iter() {
        println!("Field: {:?}", f.ident);
        f.attrs.iter().for_each(|a| {
            println!("Attr: {:?} - {}", a.path().get_ident(),
            a.path().get_ident().unwrap());
        });
        // println!("Type: {:?}", f.ty.to_token_stream());
        match &f.ty {
            | syn::Type::Array(ref_type) => {
                println!("Array type: {:?}", ref_type.to_token_stream());
            },
            | syn::Type::BareFn(ref_type) => {
                println!("Barefn type: {:?}", ref_type.to_token_stream());
            },
            | syn::Type::Group(ref_type) => {
                println!("Group: {:?}", ref_type.to_token_stream());
            },
            | syn::Type::ImplTrait(ref_type) => {
                println!("Impl trait: {:?}", ref_type.to_token_stream());
            },
            | syn::Type::Infer(ref_type) => {
                println!("Infer type: {:?}", ref_type.to_token_stream());
            },
            | syn::Type::Macro(ref_type) => {
                println!("Macro type: {:?}", ref_type.to_token_stream());
            },
            | syn::Type::Never(ref_type) => {
                println!("Never type: {:?}", ref_type.to_token_stream());
            },
            | syn::Type::Paren(ref_type) => {
                println!("Paren type: {:?}", ref_type.to_token_stream());
            },
            | syn::Type::Path(ref_type) => {
                println!("Path type: {:?}", ref_type.to_token_stream());
            },
            | syn::Type::Reference(ref_type) => {
                println!("Ref type: {:?}", ref_type.to_token_stream());
            },
            | syn::Type::Slice(ref_type) => {
                println!("Slice type: {:?}", ref_type.to_token_stream());
            },
            | syn::Type::TraitObject(ref_type) => {
                println!("Trait object type: {:?}", ref_type.to_token_stream());
            },
            | syn::Type::Tuple(ref_type) => {
                println!("Tuple type: {:?}", ref_type.to_token_stream());
            },
            | syn::Type::Verbatim(ref_type) => {
                println!("Verbatim type: {:?}", ref_type.to_token_stream());
            },
            | _ => {
                println!("Other type: {:?}", f.ty.to_token_stream());
            },
        }
    } */

    /*
    let data_expanded_members = fields.named.into_iter().map(|field| {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        let field_name_stringified =
            LitStr::new(&field_name.to_string(), span);
        quote_spanned! { span =>
            make_number(#field_name_stringified, &self.#field_name)
        }

    });
    quote! {
        impl RuleSystem for #name {
            fn do_something(self: &'_ Self) -> Output {
                Output {
                    data: vec![
                        #(#data_expanded_members ,)*
                    ],
                }
            }
        }
    }
    */

/*
#[proc_macro_derive(DynamicServices)]
pub fn dynamic_services_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_dynamic_services(&ast)
}

fn impl_dynamic_services(ast: &syn::DeriveInput) -> TokenStream {
    println!("Given ast: {:?}", quote!(#ast));

    let name = &ast.ident;
    let gen = quote! {
        impl #name<'_> {
            pub fn blah(&self) {
                println!("blah");
            }
        }
        // impl HelloMacro for #name {
        //     fn hello_macro() {
        //         println!("Hello, Macro! My name is {}!", stringify!(#name));
        //     }
        // }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn dynamic_services_inject(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
    // quote!{}.into()
}
 */