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
        fn name() -> &'static str {
            MODULE.name()
        }

        #[no_mangle]
        fn desc() -> &'static str {
            MODULE.desc()
        }

        #[no_mangle]
        fn id() -> u64 {
            MODULE.id()
        }

        #[no_mangle]
        fn version() -> u64 {
            MODULE.version()
        }

        #[no_mangle]
        fn supported_versions() -> &'static [u64]{
            MODULE.supported_versions()
        }

        #[no_mangle]
        fn default_element_settings() -> Settings {
            MODULE.default_element_settings()
        }

        #[no_mangle]
        fn default_location_settings() -> Settings {
            MODULE.default_location_settings()
        }

        #[no_mangle]
        fn poll_element(ctx: &mut std::task::Context, element: Arc<RwLock<Element>>, storage: &mut Storage) -> Result<(), SessionError> {
            MODULE.poll_element(ctx, element, storage)
        }

        #[no_mangle]
        fn poll_location(ctx: &mut std::task::Context, location: Arc<RwLock<Location>>, storage: &mut Storage) -> Result<(), SessionError> {
            MODULE.poll_location(ctx, location, storage)
        }

        #[no_mangle]
        fn element_on_event(
            element: std::sync::Arc<std::sync::RwLock<Element>>,
            event: Event,
            storage: &mut Storage,
        ) -> SessionResult<()> {
            MODULE.element_on_event(element, event, storage)
        }

        #[no_mangle]
        fn location_on_event(
            location: std::sync::Arc<std::sync::RwLock<Location>>,
            event: Event,
            storage: &mut Storage,
        ) -> SessionResult<()> {
            MODULE.location_on_event(location, event, storage)
        }

        #[no_mangle]
        fn supports_protocols() -> &'static [&'static str] {
            MODULE.supports_protocols()
        }

        #[no_mangle]
        fn supports_extensions() -> &'static [&'static str] {
            MODULE.supports_extensions()
        }
    }
    .into()
}
