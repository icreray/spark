mod vertex;

use quote::quote;
use proc_macro::TokenStream;
use syn::DeriveInput;
use crate::vertex::unsafe_impl_bytemuck_types;

#[proc_macro_attribute]
pub fn vertex(_arguments: TokenStream, item: TokenStream) -> TokenStream {
    let item = proc_macro2::TokenStream::from(item);
    quote!(
        #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Vert)]
        #item
    ).into()
}

#[proc_macro_attribute]
pub fn unsafe_vertex(_arguments: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse::<DeriveInput>(item.clone()).unwrap();
    let unsafe_impl = unsafe_impl_bytemuck_types(ast);
    let item = proc_macro2::TokenStream::from(item);
    quote!(
        #[derive(Copy, Clone, Vert)]
        #item

        #unsafe_impl
    ).into()
}

#[proc_macro_derive(Vert, attributes(attribute))]
pub fn derive_vertex(input: TokenStream) -> TokenStream {
    let ast = syn::parse::<DeriveInput>(input).unwrap();
    vertex::derive_vertex(ast)
        .unwrap_or_else(|err| err.into_compile_error()).into()
}
