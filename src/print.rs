pub struct OPrint;
impl OPrint {
    pub fn test_print() {
        // use println
        println!("this message print by println");

        // use print
        print!("this message print by print");

        // print {}
        println!("{{}}");

        // print variable
        let a = 1;
        let b = 2;
        println!("a is {}", a);

        // print multi variable
        println!("a is {0}, b is {1}", a, b);
    }
}
