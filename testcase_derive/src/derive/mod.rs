mod parse;

use parse::{field_ident, iter_field_blocks, iter_fields, FieldBlock};

use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn impl_testcase_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let parse_fields: TokenStream = iter_field_blocks(ast)
        .map(|field| match field {
            FieldBlock::Inline(fields) => {
                let fields: Vec<_> = fields.iter().map(|field| field_ident(*field)).collect();

                quote! {
                    let (#(#fields),*) = reader
                        .parse_line()
                        .map_err(|err| {
                            format!("invalid value for {}: {}", stringify!(#(#fields),*), err)
                        })?;
                }
            }
            FieldBlock::Lines { field, count } => {
                let field = field_ident(field);

                quote! {
                    let #field = reader
                        .parse_lines(#count)
                        .map_err(|err| {
                            format!("invalid value for {}: {}", stringify!(#field), err)
                        })?;
                }
            }
        })
        .collect();

    let all_fields: Vec<_> = iter_fields(ast).map(field_ident).collect();

    quote! {
        impl testcase::prelude::TestCase for #name {
            fn parse(reader: &mut impl std::io::BufRead) -> Result<Self, testcase::prelude::Error> {
                use testcase::prelude::*;

                #parse_fields
                Ok(Self { #(#all_fields),* })
            }
        }
    }
}
