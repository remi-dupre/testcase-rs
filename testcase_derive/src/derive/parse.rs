/// Iterate over the fields of the structure.
pub(super) fn iter_fields(ast: &syn::DeriveInput) -> impl Iterator<Item = &syn::Field> {
    let data_struct = match &ast.data {
        syn::Data::Struct(data) => data,
        _ => panic!("TestCase can only be implemented for structs"),
    };

    let fields = match &data_struct.fields {
        syn::Fields::Named(fields) => &fields.named,
        _ => panic!("TestCase fields must have names"),
    };

    fields.iter()
}

pub(super) fn field_ident(field: &syn::Field) -> &syn::Ident {
    field
        .ident
        .as_ref()
        .expect("TestCase fields must have identifiers")
}

// ---
// --- FieldBlock
// ---

/// A block that can be parsed from a reader.
pub(super) enum FieldBlock<'a> {
    Inline(Vec<&'a syn::Field>),
    Lines {
        field: &'a syn::Field,
        count: Box<syn::Expr>,
    },
}

/// Iterate over parsed blocks for the structure.
pub(super) fn iter_field_blocks(ast: &syn::DeriveInput) -> impl Iterator<Item = FieldBlock> {
    let mut fields = iter_fields(ast).peekable();

    std::iter::from_fn(move || {
        let next_attribute = Attribute::for_field(fields.peek()?).unwrap_or_else(|| {
            panic!(
                "TestCase field {:?} missing attribute",
                fields.peek().unwrap().ident
            )
        });

        Some({
            match next_attribute {
                Attribute::Inline => {
                    let mut tuple = vec![fields.next().unwrap()];

                    while matches!(fields.peek().copied().map(Attribute::for_field), Some(None)) {
                        tuple.push(fields.next().unwrap());
                    }

                    FieldBlock::Inline(tuple)
                }
                Attribute::Lines { count } => FieldBlock::Lines {
                    field: fields.next().unwrap(),
                    count,
                },
            }
        })
    })
}

// ---
// --- Attributes
// ---

/// Attributes that can be applied to a field.
enum Attribute {
    Inline,
    Lines { count: Box<syn::Expr> },
}

impl Attribute {
    fn for_field(field: &syn::Field) -> Option<Self> {
        let meta = field
            .attrs
            .iter()
            .map(|attr| attr.parse_meta().expect("Invalid TestCase attribute"))
            .find_map(|meta| match meta {
                syn::Meta::List(list) => {
                    if list.path.get_ident()? == "testcase" {
                        Some(list.nested)
                    } else {
                        None
                    }
                }
                _ => None,
            })?
            .into_iter()
            .next()
            .expect("Empty TestCase attribute");

        let meta = match meta {
            syn::NestedMeta::Meta(inner) => inner,
            _ => panic!("Invalid TestCase attribute parameters"),
        };

        Some(match meta {
            syn::Meta::Path(path) if path.get_ident()? == "line" => Self::Inline,
            syn::Meta::NameValue(syn::MetaNameValue {
                path,
                eq_token: _,
                lit: syn::Lit::Str(val),
            }) if path.get_ident()? == "lines" => Self::Lines {
                count: val.parse().expect("Invalid expression in lines"),
            },
            _ => panic!("Invalid TestCase attribute parameters"),
        })
    }
}
