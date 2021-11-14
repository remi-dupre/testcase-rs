use proc_macro2::TokenStream;
use quote::{format_ident, quote};

struct TestArgs {
    input: String,
    output: String,
}

fn parse_args(args: syn::AttributeArgs) -> Option<TestArgs> {
    let mut test_in = None;
    let mut test_out = None;

    for arg in args {
        match arg {
            syn::NestedMeta::Meta(syn::Meta::NameValue(meta)) => {
                let field = {
                    match (meta.path.get_ident())
                        .expect("hackercup parameters must be a correct identifier")
                        .to_string()
                        .as_str()
                    {
                        "input" => &mut test_in,
                        "output" => &mut test_out,
                        arg => panic!("Unrecognised parameter {} for hackercup", arg),
                    }
                };

                let path = {
                    match meta.lit {
                        syn::Lit::Str(path) => path.value(),
                        _ => panic!("Test parameters expect a valid literal for hackercup"),
                    }
                };

                *field = Some(path);
            }
            _ => todo!(),
        }
    }

    match (test_in, test_out) {
        (None, None) => None,
        (Some(input), Some(output)) => Some(TestArgs { input, output }),
        _ => panic!("hackercup need a test input AND a test output"),
    }
}

fn generate_test(args: TestArgs, solution_ident: &syn::Ident) -> TokenStream {
    let test_ident = format_ident!("test_hackercup_{}", solution_ident);
    let input_path = args.input;
    let output_path = args.output;

    quote! {
        #[cfg(test)]
        mod test {
            #[test]
            fn #test_ident() {
                let mut input = std::io::BufReader::new(include_bytes!(#input_path) as &[u8]);
                let mut output = Vec::new();

                super::__hackercup_raw_solution(&mut input, &mut output)
                    .expect(concat!(stringify!(#solution_ident), " has crashed"));

                let output = String::from_utf8_lossy(&output);
                let expected = include_str!(#output_path);

                for (out, exp) in output.lines().zip(expected.lines()) {
                    assert_eq!(out, exp);
                }
            }
        }

    }
}

pub(crate) fn generate(args: syn::AttributeArgs, solution: syn::ItemFn) -> TokenStream {
    let solution_ident = &solution.sig.ident;

    if solution_ident == "main" {
        panic!("hackercup solution can't be called main")
    }

    let tests = parse_args(args)
        .map(|args| generate_test(args, solution_ident))
        .unwrap_or_else(|| quote! {});

    quote! {
        #solution

        fn __hackercup_raw_solution(
            input: &mut impl std::io::BufRead,
            output: &mut impl std::io::Write
        ) -> Result<(), testcase::Error> {
            use testcase::prelude::*;

            let cases: u64 = input.parse_line()?;

            for case in 1..=cases {
                let input = input.parse_testcase()?;
                let result = #solution_ident(input);
                writeln!(output, "Case #{}: {}", case, result)
                    .expect("failed while writing output");
            }

            Ok(())
        }

        fn main() -> Result<(), testcase::Error> {
            let input = std::io::stdin();
            let mut input = input.lock();

            let output = std::io::stdout();
            let mut output = output.lock();

            __hackercup_raw_solution(&mut input, &mut output)
        }

        #tests
    }
}
