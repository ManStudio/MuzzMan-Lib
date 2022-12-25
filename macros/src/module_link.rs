use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::quote;

pub fn module_link(
    _meta: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();

    let mut name = None;

    {
        let mut struct_finded = false;

        for token in input.clone() {
            match token {
                TokenTree::Ident(test) => {
                    if struct_finded {
                        name = Some(test);
                        break;
                    }
                    if test == Ident::new("struct", Span::call_site()) {
                        struct_finded = true
                    }
                }
                _ => {}
            }
        }
    }

    let name = name.expect("is not struct");

    quote! {
        #input

        static MODULE: #name = #name;

        #[no_mangle]
        fn init(info: MInfo) -> Result<(), String> {
            MODULE.init(info)
        }

        #[no_mangle]
        fn get_name() -> String {
            MODULE.get_name()
        }

        #[no_mangle]
        fn get_desc() -> String {
            MODULE.get_desc()
        }

        #[no_mangle]
        fn init_settings(data: &mut Data) {
            MODULE.init_settings(data)
        }

        #[no_mangle]
        fn init_element_settings(data: &mut Data) {
            MODULE.init_element_settings(data)
        }

        #[no_mangle]
        fn init_element(element: ERow) {
            MODULE.init_element(element)
        }
        #[no_mangle]
        fn step_element(element: ERow, control_flow: &mut ControlFlow, storage: &mut Storage) {
            MODULE.step_element(element, control_flow, storage)
        }

        #[no_mangle]
        fn accept_extension(filename: &str) -> bool {
            MODULE.accept_extension(filename)
        }

        #[no_mangle]
        fn accept_url(url: Url) -> bool {
            MODULE.accept_url(url)
        }

        #[no_mangle]
        fn init_location(location: LInfo, data: FileOrData) {
            MODULE.init_location(location, data)
        }
    }
    .into()
}