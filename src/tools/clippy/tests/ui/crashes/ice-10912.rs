#![warn(clippy::unreadable_literal)]
//@no-rustfix
fn f2() -> impl Sized { && 3.14159265358979323846E }
//~^ ERROR: expected at least one digit in exponent
//~| ERROR: long literal lacking separators
//~| NOTE: `-D clippy::unreadable-literal` implied by `-D warnings`

fn main() {}
