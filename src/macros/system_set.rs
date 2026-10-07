use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn derive_system_set_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics avenix::app::system::system_set::SystemSet for #name #ty_generics #where_clause {
            fn clone_box(&self) -> std::boxed::Box<dyn avenix::app::system::system_set::SystemSet> {
                Box::new(std::clone::Clone::clone(self))
            }

            fn create_guard_node(
                &self,
                gates: Vec<avenix::app::system::condition::ConditionFn>,
                shared_flag: std::sync::Arc<std::sync::atomic::AtomicBool>,
            ) -> avenix::app::schedule::SystemNode {
                avenix::app::schedule::SystemNode::new(avenix::app::system::system_set::GuardNode::<Self>::new(gates, shared_flag))
            }
        }
    };

    TokenStream::from(expanded)
}
