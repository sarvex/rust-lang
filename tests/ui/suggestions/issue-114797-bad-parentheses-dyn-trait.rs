//run-rustfix
#![allow(dead_code)]

trait Trait {}

fn assert_send(ptr: *mut dyn Trait) -> *mut dyn (Trait + Send) {
    //~^ ERROR incorrect parentheses around trait bounds
    ptr as _
}

fn foo2(_: &dyn (Trait + Send)) {}
//~^ ERROR incorrect parentheses around trait bounds

fn foo3(_: &dyn(Trait + Send)) {}
//~^ ERROR incorrect parentheses around trait bounds

fn main() {}
