use proc_macro::{TokenStream};
use quote::{quote, quote_spanned};
use syn::{self, DataStruct, LitStr};
use syn::{Data, DataEnum, DataUnion, DeriveInput, Error, Fields, Result, token};
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
    println!("Given ast: {:?}", quote!(#ast));
    println!("Name {} attrs {}", ast.ident, ast.attrs.len());

    let _ = TokenStream::from(match impl_my_trait(ast.clone()) {
        | Ok(it) => it,
        | Err(err) => err.to_compile_error(),
    });

    let name = ast.ident;
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


fn impl_my_trait (ast: DeriveInput)
  -> Result<TokenStream2>
{ Ok ({
    let name = ast.ident;
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

    for f in fields.named.iter() {
        println!("Field: {:?}", f.ident);
        f.attrs.iter().for_each(|a| {
            println!("Attr: {:?} - {}", a.path().get_ident(),
            a.path().get_ident().unwrap());
        });
    }

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
})}
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