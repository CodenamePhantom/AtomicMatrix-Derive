use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields};

#[proc_macro_derive(SafeSHM)]
pub fn derive_safe_shm(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    for param in input.generics.type_params_mut() {
        param.bounds.push(parse_quote!(atomic_matrix::helpers::safe_shm::SafeSHM));
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let field_types = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => fields.named.iter().map(|f| &f.ty).collect::<Vec<_>>(),
            Fields::Unnamed(fields) => fields.unnamed.iter().map(|f| &f.ty).collect::<Vec<_>>(),
            Fields::Unit => Vec::new(),
        }
        _ => panic!("SafeSHM can only be derived for structs!"),
    };

    let assertions = field_types.iter().map(|ty| {
        quote! {
            assert_field_implements_safe_shm::<#ty>();
        }
    });

    let expanded = quote! {
        unsafe impl #impl_generics atomic_matrix::helpers::safe_shm::SafeSHM for #name #ty_generics #where_clause {}

        #[allow(dead_code)]
        const _: () = {
            fn assert_field_implements_safe_shm<Field: atomic_matrix::helpers::safe_shm::SafeSHM>() {}

            fn __safe_shm_field_check #impl_generics (_: &#name #ty_generics) #where_clause {
                #(#assertions)*
            }
        };
    };

    TokenStream::from(expanded)
}