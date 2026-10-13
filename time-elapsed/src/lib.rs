//! Print the elapsed time of a function with an attribute macro.
//!
//! # Example
//!
//! ```
//! use mi_time_elapsed::time_elapsed;
//!
//! #[time_elapsed]
//! fn fetch_value() -> u32 {
//!     42
//! }
//!
//! assert_eq!(fetch_value(), 42);
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Prints the elapsed time after a function completes.
///
/// Both synchronous and asynchronous functions are supported. For an async
/// function, the timing is printed when its future completes or is dropped.
/// The macro does not depend on an async runtime.
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

    let attrs = &function.attrs;
    let vis = &function.vis;
    let sig = &function.sig;
    let name = &sig.ident;
    let body = &function.block;

    quote! {
        #(#attrs)*
        #vis #sig {
            struct __TimeElapsedGuard {
                started_at: ::std::time::Instant,
                name: &'static str,
            }

            impl ::core::ops::Drop for __TimeElapsedGuard {
                fn drop(&mut self) {
                    ::std::println!(
                        "[time_elapsed] {} took {:?}",
                        self.name,
                        self.started_at.elapsed()
                    );
                }
            }

            let __time_elapsed_guard = __TimeElapsedGuard {
                started_at: ::std::time::Instant::now(),
                name: ::core::stringify!(#name),
            };
            let _ = &__time_elapsed_guard;
            #body
        }
    }
    .into()
}
