use proc_macro::TokenStream;
use quote::quote;
use syn;

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
        impl #name {
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