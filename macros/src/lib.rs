mod vertex;

use quote::quote;
use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_attribute]
pub fn vertex(_arguments: TokenStream, item: TokenStream) -> TokenStream {
    let item = proc_macro2::TokenStream::from(item);
    quote!(
        #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Vertex)]
        #item
    ).into()
}

#[proc_macro_derive(Vertex, attributes(attribute))]
pub fn derive_vertex(input: TokenStream) -> TokenStream {
    let ast = syn::parse::<DeriveInput>(input).unwrap();
    vertex::derive_vertex(ast)
        .unwrap_or_else(|err| err.into_compile_error()).into()
}
