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

fn impl_struct(name_impl: Ident, generics: Generics, fields: Fields) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let count = 0_usize..;
    let field_types = fields.iter().map(|field| field.ty.to_token_stream());
    let field_name_impls = fields
        .iter()
        .enumerate()
        .map(|(num, field)| field.ident.clone().map(|x| x.to_string()).unwrap_or_else(|| format!("{}", num)));
    let name_impl_str = name_impl.to_string();

    quote! {
        impl #impl_generics ::type_trait::r#type::Type for #name_impl #ty_generics #where_clause {
            fn type_info() -> ::type_trait::r#type::TypeInfoVtable {
                struct TypeInfoStruct;

                impl ::type_trait::r#type::TypeInfoImpl for TypeInfoStruct {
                    fn member_by_index_impl(w: usize) -> Option<(&'static str, ::type_trait::r#type::TypeInfoVtable)> {
                        match w {
                            #(
                                #count => {
                                    Some((#field_name_impls, <#field_types as ::type_trait::r#type::Type>::type_info()))
                                }
                            ),*
                            _ => {
                                None
                            }
                        }
                    }

                    fn name_impl() -> Option<&'static str> {
                        Some(#name_impl_str)
                    }

                    fn type_type_impl() -> ::type_trait::r#type::TypeType {
                        ::type_trait::r#type::TypeType::Struct
                    }

                    fn new() -> TypeInfoStruct {
                        TypeInfoStruct
                    }
                }

                Box::new(TypeInfoStruct)
            }
        }
    }
}

fn impl_enum(name_impl: Ident, generics: Generics, attrs: Vec<Attribute>, data: DataEnum) -> TokenStream {
    panic!("");
}

fn impl_unit_struct(name_impl: Ident, generics: Generics) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let name_impl_str = name_impl.to_string();

    quote! {
        impl #impl_generics ::type_trait::r#type::Type for #name_impl #ty_generics #where_clause {
            fn type_info() -> ::type_trait::r#type::TypeInfoVtable {
                struct TypeInfoStruct;

                impl ::type_trait::r#type::TypeInfoImpl for TypeInfoStruct {
                    fn member_by_index_impl(_: usize) -> Option<(&'static str, ::type_trait::r#type::TypeInfoVtable)> {
                        None
                    }

                    fn name_impl() -> Option<&'static str> {
                        Some(#name_impl_str)
                    }

                    fn type_type_impl() -> ::type_trait::r#type::TypeType {
                        ::type_trait::r#type::TypeType::Struct
                    }

                    fn new() -> TypeInfoStruct {
                        TypeInfoStruct
                    }
                }

                Box::new(TypeInfoStruct)
            }
        }
    }
}
