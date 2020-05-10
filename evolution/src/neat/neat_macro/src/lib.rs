extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::Data::Struct;
use syn::{Attribute, DeriveInput, Field, Fields};

#[proc_macro_derive(GetCore, attributes(core))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    if attrs_contains(&ast.attrs, "core") {
        impl_getcore_self_macro(&ast)
    } else {
        impl_getcore_macro(&ast)
    }
}

fn impl_getcore_self_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl GetCore<Self> for #name {
            fn get_core(&self) -> &Self {
                self
            }

            fn get_core_mut(&mut self) -> &mut Self {
                self
            }
        }
    };
    gen.into()
}

fn impl_getcore_macro(ast: &syn::DeriveInput) -> TokenStream {
    let field = find_field(ast, "core").expect("no field declared as core");
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;

    let name = &ast.ident;
    let gen = quote! {
        impl GetCore<#field_type> for #name {
            fn get_core(&self) -> &#field_type {
                &self.#field_name
            }

            fn get_core_mut(&mut self) -> &mut #field_type {
                &mut self.#field_name
            }
        }
    };
    gen.into()
}

fn find_field<'a>(ast: &'a syn::DeriveInput, attr_name: &'static str) -> Option<&'a Field> {
    match ast.data {
        Struct(ref ds) => match ds.fields {
            Fields::Named(ref fields) => fields
                .named
                .iter()
                .filter(|field| attrs_contains(&field.attrs, attr_name))
                .next(),
            _ => None,
        },
        _ => None,
    }
}

fn attrs_contains(attrs: &Vec<Attribute>, attr_name: &'static str) -> bool {
    attrs
        .iter()
        .filter(|attr| attr.path.segments[0].ident == attr_name)
        .next()
        .is_some()
}
