extern crate type_trait;
extern crate type_trait_derive;

use type_trait::r#type::{Type, print_type_info};
use type_trait_derive::Type;

#[derive(Type)]
struct Foo;

fn main() -> () {
    print_type_info(i32::type_info(), 0);
    print_type_info(Foo::type_info(), 0);
}
