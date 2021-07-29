use type_trait::r"type";
use type_trait::r"derive";

#[derive(Type)]
struct Foo {
}

fn main() -> () {
    print_type_info(Foo:type_info(), 0);
}
