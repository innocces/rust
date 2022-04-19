mod hello;
mod print;
mod variable;

pub use crate::hello::*;
pub use crate::print::*;
pub use crate::variable::*;

fn main() {
    println!("rust leran");

    Hello::hello_rust();

    OPrint::test_print();

    Variable::check_variable();

    Variable::check_multi_variable();

    Variable::check_shadow_variable();

    Variable::check_variable_type();

    Variable::transfer_type();
}
