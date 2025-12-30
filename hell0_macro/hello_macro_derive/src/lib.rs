extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
use syn::{self, Data};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let g = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, {}",stringify!(#name));
            }
        }
    };
    g.into()
}

#[proc_macro_derive(MyDefault)]
pub fn my_default(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let id = ast.ident;

    let Data::Struct(s) = ast.data else {
        panic!("must in struct");
    };
    let mut field_ast = quote!();
    for (idx, f) in s.fields.iter().enumerate() {
        let (field_id, field_ty) = (&f.ident, &f.ty);
        if field_id.is_none() {
            let field_idx = syn::Index::from(idx);
            field_ast.extend(quote! {});
        } else {
            field_ast.extend(quote! {});
        }
    }
    quote! {
        impl Default for # id {
            fn default() -> Self {
                Self{}
            }
        }
    }
    .into()
}
