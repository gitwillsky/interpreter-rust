use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(NewFunction)]
pub fn new_function(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // 获取字段
    let fields = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(FieldsNamed { named, .. }) => named,
            _ => panic!("Expr macro can only be derived for struct with named fields"),
        },
        _ => panic!("Expr macro can only be derived for struct"),
    };

    // 生成构造函数参数
    let ctor_args = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        quote! { #name: #ty }
    });

    // 生成构造函数初始化字段
    let field_inits: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|field| {
            let name = &field.ident;
            quote! { #name }
        })
        .collect();

    let expanded = quote! {
        impl #struct_name {
            pub fn new(#(#ctor_args,)*) -> Self {
                Self {
                    #(#field_inits,)*
                }
            }
        }
    };

    expanded.into()
}
