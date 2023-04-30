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
            if let TokenTree::Ident(test) = token {
                if struct_finded {
                    name = Some(test);
                    break;
                }
                if test == Ident::new("struct", Span::call_site()) {
                    struct_finded = true
                }
            }
        }
    }

    let name = name.expect("is not struct");

    quote! {
        #input

        static MODULE: #name = #name;

        #[no_mangle]
        fn init(info: MRef) -> Result<(), String> {
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
        fn get_uid() -> UID{
            MODULE.get_uid()
        }

        #[no_mangle]
        fn get_version() -> String{
            MODULE.get_version()
        }

        #[no_mangle]
        fn supported_versions() -> Range<u64>{
            MODULE.supported_versions()
        }

        #[no_mangle]
        fn init_settings(data: &mut Values) -> Result<(), SessionError> {
            MODULE.init_settings(data)
        }

        #[no_mangle]
        fn init_element_settings(data: &mut Values) -> Result<(), SessionError> {
            MODULE.init_element_settings(data)
        }

        #[no_mangle]
        fn init_element(element: ERow) -> Result<(), SessionError> {
            MODULE.init_element(element)
        }
        #[no_mangle]
        fn step_element(element: ERow, control_flow: &mut ControlFlow, storage: &mut Storage) -> Result<(), SessionError> {
            MODULE.step_element(element, control_flow, storage)
        }

        #[no_mangle]
        fn step_location(location: LRow, control_flow: &mut ControlFlow, storage: &mut Storage) -> Result<(), SessionError> {
            MODULE.step_location(location, control_flow, storage)
        }

        #[no_mangle]
        fn accept_extension(filename: &str) -> bool {
            MODULE.accept_extension(filename)
        }

        #[no_mangle]
        fn accepted_protocols() -> Vec<String>{
            MODULE.accepted_protocols()
        }

        #[no_mangle]
        fn accept_url(url: String) -> bool {
            MODULE.accept_url(url)
        }

        #[no_mangle]
        fn accepted_extensions() -> Vec<String>{
            MODULE.accepted_extensions()
        }

        #[no_mangle]
        fn init_location(location: LRef) -> Result<(), SessionError> {
            MODULE.init_location(location)
        }

        #[no_mangle]
        fn notify(_ref: Ref, event: Event) -> Result<(), SessionError> {
            MODULE.notify(_ref, event)
        }
    }
    .into()
}
