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

    pub fn check_variable_type() {
      let x: i16 = -1_2_34;
      let xx = -1_2_34i16;
      let y: u32 = 1234567899;
      let z: f32 = 1.2345_6789;
      let u: bool = true;
      let v: char = '中';
      println!("i16: {}", x);
      println!("i16xx: {}", xx);
      println!("u32: {}", y);
      println!("f32: {}", z);
      println!("bool: {}", u);
      println!("char(非英文字符): {}", v);

      // 字符串
      let s = String::from("hello");
      let mut s1 = "hello".to_string();
      s1.push(' ');
      s1.push_str("rust");
      println!("{}, {}", s, s1);

      // tuple
      let t = (1, 2, 3, 4, 5);
      println!("{:?}", t);
      println!("{}", t.1);
      let un: () = ();
      println!("{:?}", un);

      // array
      let array = [1, 2, 3, 4, 5];
      let array_type: [u8; 5] = [1, 2, 3, 4,  5];
      let array_100 = [0_i8; 100];
      println!("{:?}, {:?}", array, array_100);
      for i in array_type.iter() {
        println!("{}", i);
      }
      for l in &array_type {
        println!("{}", l);
      }

      // 引用
      let q = 33;
      let q_ref = &q;
      let q_ref_1 = q_ref;
      println!("{}", std::ptr::eq(q_ref, q_ref_1));

      // 可变引用
      let mut q_mut = 33;
      let q_mut_ref = &mut q_mut;
      println!("{}", q_mut_ref);

      // 解引用
      let x_i = 1;
      let x_i_ref = &x_i;

      assert_eq!(x_i, *x_i_ref);

      // Slice
      let slice_val: [u8; 5] = [1, 2, 3, 4, 5];
      let slice_val_1 = &slice_val[0..2];
      println!("{:?}", slice_val_1);
    }

    pub fn transfer_type() {
      let a: i32 = -1_234;
      let b = a as u8;
      println!("i32: {0}, u8 is {1}", a, b);
      let thruth = true as u32;
      let falsy = false as u32;
      println!("thruth: {0}, falsy is {1}", thruth, falsy);
    }
}
