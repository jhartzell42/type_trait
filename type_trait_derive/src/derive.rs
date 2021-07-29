use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{self, Attribute, Data, DataEnum, DeriveInput, Fields, Generics, Ident};

pub fn expand_derive(ast: DeriveInput) -> TokenStream {
    match ast.data {
        Data::Struct(ds) => match ds.fields {
            Fields::Named(_) | Fields::Unnamed(_) => {
                impl_struct(ast.ident, ast.generics, ds.fields)
            }
            Fields::Unit => impl_unit_struct(ast.ident, ast.generics),
        },
        Data::Enum(data) => {
            impl_enum(ast.ident, ast.generics, ast.attrs, data)
        },
        _ => panic!("Only structures and enums supported at the moment"),
    }
}

fn impl_struct(name: Ident, generics: Generics, fields: Fields) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let count = 0_usize..;
    let field_types = fields.iter().map(|field| field.ty.to_token_stream());
    let field_names = fields
        .iter()
        .enumerate()
        .map(|(num, field)| field.ident.clone().map(|x| x.to_string()).unwrap_or_else(|| format!("{}", num)));
    let name_str = name.to_string();

    quote! {
        impl #impl_generics ::type_trait::r#type::Type for #name #ty_generics #where_clause {
            fn type_info() -> ::type_trait::r#type::TypeInfoVtable {
                struct TypeInfoStruct;

                impl ::type_trait::r#type::TypeInfo for TypeInfoStruct {
                    fn member_by_index(&self, w: usize) -> Option<(&'static str, ::type_trait::r#type::TypeInfoVtable)> {
                        match w {
                            #(
                                #count => {
                                    Some((#field_names, <#field_types as ::type_trait::r#type::Type>::type_info()))
                                }
                            ),*
                            _ => {
                                None
                            }
                        }
                    }

                    fn name(&self) -> Option<&'static str> {
                        Some(#name_str)
                    }

                    fn type_type(&self) -> ::type_trait::r#type::TypeType {
                        ::type_trait::r#type::TypeType::Struct
                    }

                    fn copy(&self) -> ::type_trait::r#type::TypeInfoVtable {
                        Box::new(TypeInfoStruct)
                    }
                }

                Box::new(TypeInfoStruct)
            }
        }
    }
}

fn impl_enum(name: Ident, generics: Generics, attrs: Vec<Attribute>, data: DataEnum) -> TokenStream {
    panic!("");
}

fn impl_unit_struct(name: Ident, generics: Generics) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let name_str = name.to_string();

    quote! {
        impl #impl_generics ::type_trait::r#type::Type for #name #ty_generics #where_clause {
            fn type_info() -> ::type_trait::r#type::TypeInfoVtable {
                struct TypeInfoStruct;

                impl ::type_trait::r#type::TypeInfo for TypeInfoStruct {
                    fn member_by_index(&self, _: usize) -> Option<(&'static str, ::type_trait::r#type::TypeInfoVtable)> {
                        None
                    }

                    fn name(&self) -> Option<&'static str> {
                        Some(#name_str)
                    }

                    fn type_type(&self) -> ::type_trait::r#type::TypeType {
                        ::type_trait::r#type::TypeType::Struct
                    }

                    fn copy(&self) -> ::type_trait::r#type::TypeInfoVtable {
                        Box::new(TypeInfoStruct)
                    }
                }

                Box::new(TypeInfoStruct)
            }
        }
    }
}
