use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(TypeInfo)]
pub fn derive_type_info(input: TokenStream) -> TokenStream {
    type_info(input)
}

fn type_info(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let type_name = to_snake_case(name.to_string());

    let quote = quote::quote! {
        impl typeon::TypeInfo for #name {
            const TYPE_NAME: &'static str = #type_name;
        }
    };

    quote.into()
}

fn to_snake_case(input: impl AsRef<str>) -> String {
    let input: &str = input.as_ref();
    input
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if c.is_uppercase() {
                let mut s = String::new();
                if i != 0 && !input.is_empty() && input.chars().next().unwrap().is_uppercase() {
                    s.push('_');
                }
                s.push_str(&c.to_lowercase().to_string());
                std::iter::once(s)
            } else {
                std::iter::once(c.to_string())
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

