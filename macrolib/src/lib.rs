use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn attrib_macro(_args: TokenStream, _input: TokenStream) -> TokenStream {
    quote! {
      #[allow(unused)]
      fn foo() -> &'static str {
        "foo"
      }
    }
    .into()
}
