pub struct Variable;

impl Variable {
    pub fn check_variable() {
        // use mut mark a variable as mutable
        let mut a = 1;
        a = 2;
    }

    pub fn check_multi_variable() {
        let b = 1;
        let b = 2;
    }

    pub fn check_shadow_variable() {
        let x = 5;
        let x = x + 1;
        let x = x * 2;
        println!("The value of x is: {}", x);
    }
}
