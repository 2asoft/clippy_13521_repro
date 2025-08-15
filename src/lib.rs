#[allow(clippy::disallowed_macros)]
mod bar {
    #[macrolib::attrib_macro]
    fn foo() {}
}

#[allow(clippy::disallowed_macros)]
#[macrolib::attrib_macro]
fn foo() {}
