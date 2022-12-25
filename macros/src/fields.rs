use proc_macro::TokenStream;
use quote::{format_ident, quote};

pub fn fields(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;

    let q = match input.data {
        syn::Data::Struct(data) => {
            let mut var_names = Vec::new();
            for field in data.fields.iter() {
                if let Some(i) = &field.ident {
                    var_names.push(i)
                }
            }

            let mut get_field_block = Vec::new();
            let mut get_field_mut_block = Vec::new();
            let mut get_field_type_block = Vec::new();

            for (index, var) in var_names.iter().enumerate() {
                get_field_block.push(quote! {
                    if #index == id{
                        return (&self.#var as &dyn Any).downcast_ref::<T>();
                    }
                });

                get_field_mut_block.push(quote! {
                   if #index == id{
                        return (&mut self.#var as &mut dyn Any).downcast_mut::<T>();
                    }
                });

                get_field_type_block.push(quote! {
                   if #index == id{
                        return Some(self.#var.type_id());
                    }
                });
            }

            let mut var_n = Vec::new();
            for iden in var_names.iter() {
                var_n.push(iden.to_string());
            }

            quote! {
                impl Fields for #name{
                    fn get_fields(&self) -> Vec<&str>{
                        [#(#var_n),*].to_vec()
                    }

                    fn get_field<T: 'static>(&self, id: usize) -> Option<&T>{
                        #(#get_field_block)*
                        None
                    }

                    fn get_field_mut<T: 'static>(&mut self, id: usize) -> Option<&mut T>{
                        #(#get_field_mut_block)*
                        None
                    }

                    fn get_field_type(&self, id: usize) -> Option<std::any::TypeId>{
                        #(#get_field_type_block)*
                        None
                    }

                    fn is_struct(&self) -> bool{
                        true
                    }
                    fn is_enum(&self) -> bool{
                        false
                    }

                }
            }
        }
        syn::Data::Enum(data) => {
            let mut names = Vec::new();
            let mut enum_current_block = Vec::new();
            let mut enum_get_type_block = Vec::new();
            let mut enum_get_block = Vec::new();
            let mut enum_get_mut_block = Vec::new();
            let mut enum_tuple_len_block = Vec::new();
            let mut enum_set_block = Vec::new();

            for (index, variant) in data.variants.iter().enumerate() {
                names.push(variant.ident.to_string());
                let mut fields_ids = Vec::new();
                let mut fields_num = Vec::new();
                let mut vars_ty = Vec::new();
                for (index, var) in variant.fields.iter().enumerate() {
                    vars_ty.push(&var.ty);
                    fields_num.push(format_ident!("v{}", index));
                    fields_ids.push(index)
                }
                let ident = &variant.ident;
                let len = fields_num.len();
                if len > 0 {
                    enum_get_mut_block.push(quote! {
                       Self::#ident(#(#fields_num),*) => match tuple_id{#(#fields_ids => (#fields_num as &mut dyn Any).downcast_mut::<T>(),)* _=>None}
                    });
                    enum_get_block.push(quote! {
                       Self::#ident(#(#fields_num),*) => match tuple_id{#(#fields_ids => (#fields_num as &dyn Any).downcast_ref::<T>(),)* _=>None}
                    });

                    enum_current_block.push(quote! {
                       Self::#ident(#(#fields_num),*) => #index
                    });

                    enum_set_block.push(quote! {
                       #index => {unsafe {
                            #(let #fields_num = value.get_ref(#fields_ids);)*
                            #(if #fields_num.is_none(){
                                return None;
                            })*
                            #(let #fields_num = #fields_num.unwrap();)*
                            #(if #fields_num.type_id() != std::any::TypeId::of::<#vars_ty>(){
                                return None
                            })*
                            #(let #fields_num = #fields_num;)*
                            *self = Self::#ident(#(#fields_num.downcast_ref::<#vars_ty>().unwrap().to_owned()),*)}; Some(())}
                    });
                } else {
                    enum_get_mut_block.push(quote! {
                       Self::#ident => None
                    });
                    enum_get_block.push(quote! {
                       Self::#ident => None
                    });
                    enum_current_block.push(quote! {
                       Self::#ident => #index
                    });
                    enum_set_block.push(quote! {
                       #index => {*self = Self::#ident; Some(())}
                    });
                }

                enum_get_type_block.push(quote! {
                    #index => match tuple_id { #(#fields_ids => Some(std::any::TypeId::of::<#vars_ty>()),)* _=>None}
                });

                enum_tuple_len_block.push(quote! {
                    #index => #len
                });
            }

            quote! {
                impl Fields for #name{
                    fn is_struct(&self) -> bool{
                        false
                    }

                    fn is_enum(&self) -> bool{
                        true
                    }

                    fn get_fields(&self) -> Vec<&str>{
                        [#(#names),*].to_vec()
                    }

                    fn enum_is(&self, id: usize) -> bool {
                        self.enum_current() == id
                    }

                    fn enum_get_type(&self, id: usize, tuple_id: usize) -> Option<std::any::TypeId> {
                        match id{
                            #(#enum_get_type_block,)*
                            _=>None
                        }
                    }
                    fn enum_tuple_len(&self, id: usize) -> usize {
                        match id{
                            #(#enum_tuple_len_block,)*
                            _=>0
                        }
                    }

                    fn enum_get<T: 'static>(&self, tuple_id: usize) -> Option<&T> {
                        match self{
                            #(#enum_get_block,)*
                            _=>None
                        }
                    }

                    fn enum_get_mut<T: 'static>(&mut self, tuple_id: usize) -> Option<&mut T> {
                        match self{
                            #(#enum_get_mut_block,)*
                            _=>None
                        }
                    }

                    fn enum_set<T: 'static + TGetRef>(&mut self, id: usize, value: T) -> Option<()> {
                        match id{
                            #(#enum_set_block)*
                            _=>None
                        }
                    }


                    fn enum_current(&self) -> usize {
                        match self{
                            #(#enum_current_block,)*
                            _=>usize::MAX
                        }
                    }

                }
            }
        }
        syn::Data::Union(_) => todo!(),
    };

    q.into()
}
