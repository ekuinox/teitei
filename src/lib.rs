use proc_macro::TokenStream;
use quote::quote;
use syn::{ext::IdentExt, parse_macro_input, DeriveInput, ItemStruct};

// structに対してのマクロを作るので、proc_macro_deriveでマクロ名を定義する
#[proc_macro_derive(Converter, attributes(teitei))]
pub fn getter_derive(input: TokenStream) -> TokenStream {
    dbg!(&input);

    let derive_input = parse_macro_input!(input as ItemStruct);

    match generate_converter(&derive_input) {
        Ok(generated) => generated,
        Err(err) => err.to_compile_error().into(),
    }
}

fn generate_converter(item: &ItemStruct) -> Result<TokenStream, syn::Error> {
    let target_ty = item.attrs.iter().find_map(|at| {
        if at.path.get_ident().map(|a| a.to_string()) == Some("teitei".to_string()) {
            let tokens = at.tokens.clone();
            // アホ?
            tokens.into_iter().nth(0).map(|t| {
                t.to_string()
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .to_string()
            })
        } else {
            None
        }
    });
    let Some(target_ty) = target_ty else {
        return Err(syn::Error::new_spanned(item, "message"));
    };
    let target_ty: proc_macro2::TokenStream = target_ty.parse().unwrap();

    let mut get_fields = Vec::new();
    let mut fields = Vec::new();
    for field in &item.fields {
        let ident = field.ident.as_ref().unwrap();
        // let ty = &field.ty;

        get_fields.push(quote! {
            let Some(#ident) = target.#ident else { return None };
        });
        fields.push(quote! {
            #ident,
        })
    }

    let struct_name = &item.ident;
    let (impl_generics, _, where_clause) = &item.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #struct_name #where_clause {
            fn from_partial(target: #target_ty) -> Option<#struct_name> {
                #(#get_fields)*
                Some(#struct_name {
                    #(#fields)*
                })
            }
        }
    };

    Ok(expanded.into())
}
