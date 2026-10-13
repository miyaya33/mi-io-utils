//! Print the elapsed time of an async function with an attribute macro.
//!
//! # Example
//!
//! ```
//! use mi_time_elapsed::time_elapsed;
//!
//! #[time_elapsed]
//! async fn fetch_value() -> u32 {
//!     42
//! }
//!
//! let value = futures::executor::block_on(fetch_value());
//! assert_eq!(value, 42);
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Prints the elapsed time after an async function completes.
///
/// The macro does not depend on an async runtime. It measures wall-clock time
/// with [`std::time::Instant`], so it works with Tokio, async-std, or a custom
/// executor.
#[proc_macro_attribute]
pub fn time_elapsed(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::from(attr),
            "the time_elapsed attribute does not accept arguments",
        )
        .to_compile_error()
        .into();
    }

    let function = parse_macro_input!(item as ItemFn);

    if function.sig.asyncness.is_none() {
        return syn::Error::new_spanned(
            function.sig.fn_token,
            "the time_elapsed attribute can only be used on an async function",
        )
        .to_compile_error()
        .into();
    }

    let attrs = &function.attrs;
    let vis = &function.vis;
    let sig = &function.sig;
    let name = &sig.ident;
    let body = &function.block;

    quote! {
        #(#attrs)*
        #vis #sig {
            let __time_elapsed_started_at = ::std::time::Instant::now();
            let __time_elapsed_result = (async move #body).await;
            ::std::println!(
                "[time_elapsed] {} took {:?}",
                ::core::stringify!(#name),
                __time_elapsed_started_at.elapsed()
            );
            __time_elapsed_result
        }
    }
    .into()
}
