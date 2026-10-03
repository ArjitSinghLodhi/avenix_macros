extern crate proc_macro;
mod attributes;
mod macros {
    pub mod component;
    pub mod component_bundle;
    pub mod event;
    pub mod parallel_system_param;
    pub mod query_data;
    pub mod query_filter;
    pub mod resource;
    pub mod schedule_label;
    pub mod system_param;
}

use proc_macro::TokenStream;

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    macros::component::derive_component_impl(input)
}

#[proc_macro_derive(Resource)]
pub fn derive_resource(input: TokenStream) -> TokenStream {
    macros::resource::derive_resource_impl(input)
}

#[proc_macro_derive(Event)]
pub fn derive_event(input: TokenStream) -> TokenStream {
    macros::event::derive_event_impl(input)
}

#[proc_macro_derive(ComponentBundle, attributes(avenix))]
pub fn derive_bundle(input: TokenStream) -> TokenStream {
    macros::component_bundle::derive_bundle_impl(input)
}

#[proc_macro_derive(QueryFilter, attributes(avenix))]
pub fn derive_filter(input: TokenStream) -> TokenStream {
    macros::query_filter::derive_filter_impl(input)
}

#[proc_macro_derive(QueryData, attributes(avenix))]
pub fn derive_world_query(input: TokenStream) -> TokenStream {
    macros::query_data::derive_query_data_impl(input)
}

#[proc_macro_derive(SystemParam, attributes(avenix))]
pub fn derive_system_param(input: TokenStream) -> TokenStream {
    macros::system_param::derive_system_param_impl(input)
}

#[proc_macro_derive(ParallelSystemParam)]
pub fn derive_parallel_system_param(input: TokenStream) -> TokenStream {
    macros::parallel_system_param::derive_parallel_system_param_impl(input)
}

#[proc_macro_derive(ScheduleLabel)]
pub fn derive_schedule_label(input: TokenStream) -> TokenStream {
    macros::schedule_label::derive_schedule_label_impl(input)
}
