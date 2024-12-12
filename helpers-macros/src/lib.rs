extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn timeit(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input as a function
    let input_fn = parse_macro_input!(item as ItemFn);
    let _fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;

    // Generate the wrapped function with timing
    let output = quote! {
        #vis #sig {
            let start = std::time::Instant::now();
            let result = (|| #fn_block)();
            let duration = start.elapsed();
            println!("Elapsed: {:?}", duration);
            result
        }
    };

    output.into()
}
