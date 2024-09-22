use proc_macro::TokenStream;
//use proc_macro2::TokenStream;
use quote::quote;
use syn;


#[proc_macro_derive(Hello)]
pub fn hello_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_hello_trait(&ast)
}

fn impl_hello_trait(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generated = quote! {
        impl Hello for #name {
            fn hello() {
                println!("Hello from type: {}!!!", stringify!(#name));
            }
        }
    };

    generated.into()
}