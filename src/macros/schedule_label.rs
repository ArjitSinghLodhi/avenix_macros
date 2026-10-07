use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn derive_schedule_label_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics avenix::app::schedule::ScheduleLabel for #name #ty_generics #where_clause {
            fn clone_box(&self) -> Box<dyn avenix::app:schedule::ScheduleLabel {
                Box::new(self.clone())
            }
        }
    };

    TokenStream::from(expanded)
}
