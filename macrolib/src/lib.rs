use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

#[proc_macro_attribute]
pub fn attrib_macro(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input to examine attributes
    let parsed_item = parse_macro_input!(input as Item);

    // Debug output to see what attributes are available
    let attrs: &Vec<syn::Attribute> = match &parsed_item {
        Item::Fn(item_fn) => &item_fn.attrs,
        Item::Struct(item_struct) => &item_struct.attrs,
        Item::Enum(item_enum) => &item_enum.attrs,
        _ => &vec![],
    };

    // Print debug information about attributes
    eprintln!("=== PROC MACRO DEBUG ===");
    eprintln!("Number of attributes found: {}", attrs.len());
    for (i, attr) in attrs.iter().enumerate() {
        eprintln!("Attribute {}: {}", i, quote!(#attr));
    }
    eprintln!("=== END DEBUG ===");

    // Generate the same output as before but preserve information
    quote! {
      #[macrolib::attrib_macro2]
      fn foo() -> &'static str {
        "foo"
      }
    }
    .into()
}
#[proc_macro_attribute]
pub fn attrib_macro2(_args: TokenStream, _input: TokenStream) -> TokenStream {
    quote! {
      fn foo2() -> &'static str {
        "foo2"
      }
    }
    .into()
}
