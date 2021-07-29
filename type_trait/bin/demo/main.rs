extern crate type_trait;
extern crate type_trait_derive;

use type_trait::r#type::{Type, print_type_info};
use type_trait_derive::Type;

#[derive(Type)]
struct Foo {
    foo: Option<Bar>,
    bar: Option<i32>,
    baz: i32,
}

#[derive(Type)]
struct Bar {
    foo: f32,
    bar: f64,
    baz: i32,
    baz2: Baz,
}

#[derive(Type)]
struct Baz;

fn main() -> () {
    print_type_info(Foo::type_info(), 0);
}
