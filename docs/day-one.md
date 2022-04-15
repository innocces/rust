# Day-one

使用`cargo`来创建项目

```bash
$ cargo new hello-rust
```

运行项目

```bash
$ cargon run
```

## 三方包

1. 类似于`npm`, 使用 [crates](https://crates.io/)

2. 使用的三方包

```rust
use ferris_says::say;
```

最终结果

```
    Finished dev [unoptimized + debuginfo] target(s) in 1.21s
     Running `target/debug/hello-rust`
 __________________________
< Hello fellow Rustaceans! >
 --------------------------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
```

## 单独编译 rs 文件

```bash
$ rustc xxx.ts
# 会编译出xxx二进制可执行文件
$ ./xxx
```
