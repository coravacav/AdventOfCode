extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn part1(_: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    // Extract the function name
    let fn_name = &input_fn.sig.ident;
    let test_fn_name = format_ident!("{}_test", &input_fn.sig.ident);
    let static_impl_name = format_ident!("{}_static", &input_fn.sig.ident);

    // Generate new code
    let expanded = quote! {
        #input_fn

        #[allow(non_upper_case_globals)]
        #[rust_aoc_lib::linkme::distributed_slice(crate::ALL_IMPLEMENTATIONS)]
        static #static_impl_name: rust_aoc_lib::PartImplementation = rust_aoc_lib::PartImplementation::new(rust_aoc_lib::PartNum::Part1, stringify!(#fn_name), #fn_name);

        #[test]
        fn #test_fn_name () {
            let test = std::fs::read_to_string("test.txt").unwrap_or_else(|_| std::fs::read_to_string("part1.test.txt").unwrap());
            assert_eq!(#fn_name(&test), include_str!("../part1.ans.txt").parse::<usize>().unwrap_or(std::usize::MAX));
        }
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn part2(_: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    // Extract the function name
    let fn_name = &input_fn.sig.ident;
    let test_fn_name = format_ident!("{}_test", &input_fn.sig.ident);
    let static_impl_name = format_ident!("{}_static", &input_fn.sig.ident);

    // Generate new code
    let expanded = quote! {
        #input_fn

        #[allow(non_upper_case_globals)]
        #[rust_aoc_lib::linkme::distributed_slice(crate::ALL_IMPLEMENTATIONS)]
        static #static_impl_name: rust_aoc_lib::PartImplementation = rust_aoc_lib::PartImplementation::new(rust_aoc_lib::PartNum::Part2, stringify!(#fn_name), #fn_name);

        #[test]
        fn #test_fn_name () {
            let test = std::fs::read_to_string("test.txt").unwrap_or_else(|_| std::fs::read_to_string("part2.test.txt").unwrap());
            assert_eq!(#fn_name(&test), include_str!("../part2.ans.txt").parse::<usize>().unwrap_or(std::usize::MAX));
        }
    };

    expanded.into()
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
