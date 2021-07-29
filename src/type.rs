
trait Type {
    fn type_info() -> Box<dyn TypeInfo>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum TypeType {
    Enum(&'static str),
    Struct(&'static str),
    Maybe,
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
    UnixFd,
    String,
    ObjectPath,
    Signature,
}

trait TypeInfo {
    fn variant_by_index(&self, which: usize) -> Option<(&'static str, Box<dyn TypeInfo>)>;
    fn member_by_index(&self, which: usize) -> Option<(&'static str, Box<dyn TypeInfo>)>;
    fn type_type(&self) -> TypeType;
}

struct PrimitiveTypeInfo {
    pub type_type: TypeType,
}

impl TypeInfo for PrimitiveTypeInfo {
    fn type_type(&self) -> TypeType {
        self.type_type
    }

    fn member_by_index(&self, _: usize) {
        None
    }
}

fn print_type_info(info: Box<dyn TypeInfo>, indent_level: u32) -> () {
    let indent: String = (0..indent_level).map(' ').collect();
    println!("{}{:?}", indent, info.type_type());
    match info.type_type() {
        TypeType::Enum | TypeType::Struct | TypeType::Maybe | TypeType::Array => {
            for i in 0.. {
                if let Some(name, child_info) = info.member_by_index(i) {
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
