pub trait Type {
    fn type_info() -> TypeInfoVtable;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TypeType {
    Enum,
    Struct,
    Option,
    Array,
    Byte,
    Boolean,
    Int16,
    Int32,
    Int64,
    UInt16,
    UInt32,
    UInt64,
    Double,
    String,
    Unit,
}

pub type TypeInfoVtable = Box<dyn TypeInfo>;

pub fn wrap_value<T: 'static + TypeInfo>(val: T) -> TypeInfoVtable {
    Box::new(val)
}

pub trait TypeInfo {
    fn member_by_index(&self, which: usize) -> Option<(&'static str, TypeInfoVtable)>;
    fn name(&self) -> Option<&'static str>;
    fn type_type(&self) -> TypeType;
    fn copy(&self) -> TypeInfoVtable;
}

fn wrap_as_option(val: TypeInfoVtable) -> TypeInfoVtable {
    struct TypeInfoStruct(TypeInfoVtable);

    impl TypeInfo for TypeInfoStruct {
        fn member_by_index(&self, w: usize) -> Option<(&'static str, TypeInfoVtable)> {
            match w {
                0 => {
                    let vtable = match self {
                        TypeInfoStruct(vtable) => vtable
                    };
                    Some(("some", vtable.copy()))
                },
                _ => None,
            }
        }

        fn name(&self) -> Option<&'static str> {
            None
        }

        fn type_type(&self) -> TypeType {
            TypeType::Option
        }

        fn copy(&self) -> TypeInfoVtable {
            let vtable = match self {
                TypeInfoStruct(vtable) => vtable
            };
            wrap_as_option(vtable.copy())
        }
    }

    wrap_value(TypeInfoStruct(val))
}

impl<T> Type for Option<T> where T: Type {
    fn type_info() -> TypeInfoVtable {
        wrap_as_option(T::type_info())
    }
}

impl<T> Type for &T where T: Type {
    fn type_info() -> TypeInfoVtable {
        T::type_info()
    }
}

impl<T> Type for &mut T where T: Type {
    fn type_info() -> TypeInfoVtable {
        T::type_info()
    }
}

impl<T> Type for Box<T> where T: Type {
    fn type_info() -> TypeInfoVtable {
        T::type_info()
    }
}

macro_rules! primitive_type {
    ($type: ty, $value: expr) => {
        impl Type for $type {
            fn type_info() -> TypeInfoVtable {
                struct TypeInfoStruct;

                impl TypeInfo for TypeInfoStruct {
                    fn member_by_index(&self, _: usize) -> Option<(&'static str, TypeInfoVtable)> {
                        None
                    }

                    fn name(&self) -> Option<&'static str> {
                        None
                    }

                    fn type_type(&self) -> TypeType {
                        $value
                    }

                    fn copy(&self) -> TypeInfoVtable {
                        wrap_value(TypeInfoStruct)
                    }
                }

                wrap_value(TypeInfoStruct)
            }
        }
    };
}

primitive_type!{bool, TypeType::Boolean}
primitive_type!{i32, TypeType::Int32}
primitive_type!{u8, TypeType::Byte}
primitive_type!{i16, TypeType::Int16}
primitive_type!{i64, TypeType::Int64}
primitive_type!{u16, TypeType::UInt16}
primitive_type!{u32, TypeType::UInt32}
primitive_type!{u64, TypeType::UInt64}
primitive_type!{f32, TypeType::Double}
primitive_type!{f64, TypeType::Double}
primitive_type!{&str, TypeType::String}
primitive_type!{String, TypeType::String}
primitive_type!{(), TypeType::Unit}

pub fn print_type_info(info: TypeInfoVtable, indent_level: u32) -> () {
    let indent: String = (0..indent_level).map(|_| ' ').collect();
    println!("{}{:?}", indent, info.type_type());
    match info.type_type() {
        TypeType::Enum | TypeType::Struct | TypeType::Option | TypeType::Array => {
            for i in 0.. {
                if let Some((name, child_info)) = info.member_by_index(i) {
                    println!("{}  {}", indent, name);
                    print_type_info(child_info, indent_level + 4);
                } else {
                    break;
                }
            }
        },
        _ => ()
    }
}
