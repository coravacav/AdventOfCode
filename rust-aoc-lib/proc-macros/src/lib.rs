extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn};

fn part_impl(part: usize, input_fn: ItemFn) -> TokenStream {
    let fn_name = &input_fn.sig.ident;
    let wrapped_fn_name = format_ident!("{}_wrapped", &input_fn.sig.ident);
    let test_fn_name = format_ident!("{}_test", &input_fn.sig.ident);
    let static_impl_name = format_ident!("{}_static", &input_fn.sig.ident);

    let (fn_type, new_fn_type) = match &input_fn.sig.output {
        syn::ReturnType::Default => panic!("Function has no return type"),
        syn::ReturnType::Type(_, ty) => (
            quote!(#ty),
            match quote!(#ty).to_string().as_str() {
                "usize" => quote! { rust_aoc_lib::RetType::usize },
                "isize" => quote! { rust_aoc_lib::RetType::isize },
                _ => panic!("Unsupported return type"),
            },
        ),
    };

    // Adjust test file names based on part
    let test_file_name = match part {
        1 => "part1",
        2 => "part2",
        _ => panic!("Unsupported part number"),
    };

    // Generate new code
    let expanded = quote! {
        #input_fn

        fn #wrapped_fn_name(input: &str) -> rust_aoc_lib::RetType {
            #new_fn_type(#fn_name(input))
        }

        #[allow(non_upper_case_globals)]
        #[rust_aoc_lib::linkme::distributed_slice(crate::ALL_IMPLEMENTATIONS)]
        static #static_impl_name: rust_aoc_lib::PartImplementation = rust_aoc_lib::PartImplementation::new(#part, stringify!(#fn_name), #wrapped_fn_name);

        #[test]
        fn #test_fn_name () {
            let test = std::fs::read_to_string("test.txt").unwrap_or_else(|_| std::fs::read_to_string(concat!(#test_file_name, ".test.txt")).unwrap());
            assert_eq!(#fn_name(&test), include_str!(concat!("../", #test_file_name, ".ans.txt")).parse::<#fn_type>().unwrap());
        }
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn part1(_: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    part_impl(1, input_fn)
}

#[proc_macro_attribute]
pub fn part2(_: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    part_impl(2, input_fn)
}

#[proc_macro_attribute]
pub fn init(_: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    // Extract the function name
    let fn_name = &input_fn.sig.ident;
    let static_impl_name = format_ident!("{}_static", &input_fn.sig.ident);

    // Generate new code
    let expanded = quote! {
        #input_fn

        #[allow(non_upper_case_globals)]
        #[rust_aoc_lib::linkme::distributed_slice(crate::ALL_INITS)]
        static #static_impl_name: rust_aoc_lib::InitImplementation = rust_aoc_lib::InitImplementation::new(#fn_name);
    };

    expanded.into()
}
