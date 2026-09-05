use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Error};

pub fn derive_schedule_label_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    match &input.data {
        Data::Struct(data_struct) => {
            if !matches!(data_struct.fields, Fields::Unit) {
                return Error::new_spanned(
                    &data_struct.fields,
                    "ScheduleLabel can only be derived on Unit structs (e.g., `struct MyLabel;`)"
                )
                .to_compile_error()
                .into();
            }
        }
        _ => {
            return Error::new_spanned(
                &input, 
                "ScheduleLabel can only be derived on Unit structs"
            )
            .to_compile_error()
            .into();
        }
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ScheduleLabel for #name #ty_generics #where_clause {}
    };

    TokenStream::from(expanded)
}
