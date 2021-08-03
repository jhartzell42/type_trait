extern crate type_trait;
extern crate type_trait_derive;

use type_trait::r#type::{Introspectable, print_introspection_info};
use type_trait_derive::Introspectable;

#[derive(Introspectable)]
struct Foo {
    foo: Option<Bar>,
    bar: Option<i32>,
    baz: i32,
}

#[derive(Introspectable)]
struct Bar {
    foo: f32,
    bar: f64,
    baz: i32,
    baz2: Baz,
}

#[derive(Introspectable)]
struct Baz;

fn main() -> () {
    print_introspection_info(Foo::introspection_info(), 0);
}
