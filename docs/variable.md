# 数据类型

`rust`为强类型语言.

1. `let` 使用`let`进行变量声明. 可以不增加类型进行数据声明. 但不可变更数据的类型.

- 错误示例

```rs
let a = 1;
a = 2;
```

报错信息

```
let a = 1;
  |             -
  |             |
  |             first assignment to `a`
  |             help: consider making this binding mutable: `mut a`
6 |         a = 2;
  |         ^^^^^ cannot assign twice to immutable variable
```

可使用`mut`来标注当前变量为可变

```rs
// use mut mark a variable as mutable
let mut a = 1;
a = 2;
```

2. 重复声明在变量没有被使用之前。可以被重复声明。但是类型大概只能是同一个类型

```rust
let a = 1;
let a = 2;
```

3. 重影个人觉得和上面的重复命名是一样的概念。即可以重复使用变量名称。还有点循环引用前值的感觉。

```rust
fn check_shadow_variable() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}
```

和可变变量的区别应该是。可变变量是不需要重复声明的。是一直在对同一个变量进行赋值操作。嘛~ 我是这么认为的
