mod fields;
mod module_link;

#[proc_macro_derive(Fields)]
pub fn fields(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    fields::fields(input)
}

#[proc_macro_attribute]
pub fn module_link(
    _meta: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    module_link::module_link(_meta, input)
}
