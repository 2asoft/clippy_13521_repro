#[allow(clippy::disallowed_macros)]
fn bar() {}

#[allow(clippy::disallowed_macros)]
#[macrolib::attrib_macro]
fn foo() {}
