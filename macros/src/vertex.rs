use quote::quote;
use proc_macro2::TokenStream;
use syn::{
    Data, DataStruct, DeriveInput, Field, LitInt,
    Token, Ident, parse::{Parse, ParseStream}, Error
};

pub(crate) fn unsafe_impl_bytemuck_types(ast: DeriveInput) -> TokenStream {
    let name = ast.ident;
    quote!(
        unsafe impl bytemuck::Pod for #name {}
        unsafe impl bytemuck::Zeroable for #name {}
    )
}

pub(crate) fn derive_vertex(ast: DeriveInput) -> Result<TokenStream, Error> {
    let Data::Struct(DataStruct {fields, ..}) = ast.data else {
        parse_error!(ast, "Deriving Vertex is supported only for structs")
    };

    let mut attributes = Vec::<TokenStream>::with_capacity(fields.len());
    for field in fields {
        attributes.push(extract_attribute(&field)?);
    }

    let struct_name = ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let len = attributes.len();
    Ok(quote!(
        impl #impl_generics Vert for #struct_name #ty_generics #where_clause {
            fn buffer_layout<'l>() -> wgpu::VertexBufferLayout<'l> {
                const ATTRIBUTES: [wgpu::VertexAttribute; #len] = wgpu::vertex_attr_array![#(#attributes),*];
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &ATTRIBUTES
                }
            }
        }
    ))
}

fn extract_attribute(field: &Field) -> Result<TokenStream, Error> {
    let Some(attribute) = field.attrs.iter()
        .find(|a| a.path().is_ident("attribute")) else {
        parse_error!(field, "No #[attribute(<location>, <format>)] presented for field")
    };

    let VertexAttribute {location, format} = attribute.parse_args()?;

    Ok(quote!(#location => #format))
}

struct VertexAttribute {
    location: u32,
    format: Ident
}

impl Parse for VertexAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let location = input.parse::<LitInt>()?.base10_parse()?;
        input.parse::<Token!(,)>()?;
        let format = input.parse::<Ident>()?;
        Ok(Self {location, format})
    }
}

macro_rules! parse_error {
    ($span:expr, $message:expr) => {
        return Err(Error::new_spanned(span, message))
    };
}
