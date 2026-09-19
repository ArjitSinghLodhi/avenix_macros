use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

pub fn derive_bundle_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => {
            return syn::Error::new_spanned(name, "ComponentBundle can only be derived on structs")
                .to_compile_error()
                .into();
        }
    };

    let mut types = Vec::new();
    let mut destructure_patterns = Vec::new();

    match fields {
        Fields::Named(fields_named) => {
            for field in &fields_named.named {
                let f_ident = &field.ident;
                types.push(field.ty.clone());
                destructure_patterns.push(quote! { #f_ident });
            }
        }
        Fields::Unnamed(fields_unnamed) => {
            for (i, field) in fields_unnamed.unnamed.iter().enumerate() {
                types.push(field.ty.clone());
                let dummy_ident = syn::Ident::new(&format!("field_{}", i), proc_macro2::Span::call_site());
                destructure_patterns.push(quote! { #dummy_ident });
            }
        }
        Fields::Unit => {}
    }

    let field_count = types.len();

    let destructure = match fields {
        Fields::Named(_) => quote! { let Self { #(#destructure_patterns),* } = self; },
        Fields::Unnamed(_) => quote! { let Self(#(#destructure_patterns),*) = self; },
        Fields::Unit => quote! { let Self = self; },
    };

    let expanded = quote! {
        impl #impl_generics ::avenix::ecs::commands::bundle::ComponentBundle for #name #ty_generics #where_clause {
            const TYPE_IDS: &'static [::std::any::TypeId] = &[
                #( ::std::any::TypeId::of::<#types>() ),*
            ];

            #[inline(always)]
            fn get_type_ids() -> &'static [::std::any::TypeId] {
                Self::TYPE_IDS
            }

            #[inline(always)]
            fn create_empty_columns(columns: &mut ::avenix::indexmap::IndexMap<::std::any::TypeId, ::avenix::extensions::ComponentColumn, ::avenix::rustc_hash::FxBuildHasher>) {
                #(
                    let id = ::std::any::TypeId::of::<#types>();
                    columns.insert(
                        id,
                        ::avenix::extensions::ComponentColumn::new(Vec::<#types>::new()),
                    );
                )*
            }

            #[inline(always)]
            fn push_to_archetype(self, archetype: &mut ::avenix::extensions::Archetype) {
                #destructure
                unsafe {
                    #(
                        let vec_ptr = archetype.fetch_column_raw::<#types>();
                        (*vec_ptr).push(#destructure_patterns);
                    )*
                }
            }

            #[inline(always)]
            unsafe fn insert_to_archetype(self, archetype: &mut ::avenix::extensions::Archetype, row_idx: usize) {
                #destructure
                unsafe {
                    #(
                        let vec_ptr = archetype.fetch_column_raw::<#types>();
                        let vec_ref = &mut *vec_ptr;
                        if row_idx < vec_ref.len() {
                            ::std::ptr::drop_in_place(&mut vec_ref[row_idx]);
                            ::std::ptr::write(&mut vec_ref[row_idx], #destructure_patterns);
                        } else {
                            vec_ref.push(#destructure_patterns);
                        }
                    )*
                }
            }

            type NamesArray = [&'static str; #field_count];

            #[inline(always)]
            fn get_type_names() -> Self::NamesArray {
                [
                    #( ::std::any::type_name::<#types>() ),*
                ]
            }
        }
    };

    TokenStream::from(expanded)
}
