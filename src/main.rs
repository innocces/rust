mod hello;
mod print;

pub use crate::hello::*;
pub use crate::print::*;

fn main() {
    println!("rust leran");

    Hello::hello_rust();

    OPrint::test_print();
}
