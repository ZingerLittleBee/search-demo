use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_attribute]
pub fn expensive_log(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = syn::parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input_fn.sig.ident;
    let arguments = &input_fn.sig.inputs;
    let fn_block = &input_fn.block;
    let fn_vis = &input_fn.vis;
    let asyncness = &input_fn.sig.asyncness;
    let fn_output = &input_fn.sig.output;

    let expanded = quote! {
        #fn_vis #asyncness fn #fn_name(#arguments) #fn_output {
            let start = std::time::Instant::now();
            info!("Entering function: {}", stringify!(#fn_name));
            let result = #fn_block;
            let elapsed = start.elapsed().as_millis();
            info!("Exiting function: {} (took {} ms)", stringify!(#fn_name), elapsed);
            result
            
        }
    };

    TokenStream::from(expanded)
}
