# 性能测试

单元测试是用来校验程序的正确性的，然而，程序能正常运行后，往往还需要测试程序（一部分）的执行速度，这时，f就需要用到性能测试。
通常来讲，所谓性能测试，指的是测量程序运行的速度，即运行一次要多少时间（通常是执行多次求平均值）。Rust 竟然连这个特性都集成在语言基础特性中，真的是一门很重视工程性的语言。

下面直接说明如何使用。

```
cargo new benchit
cd benchit
```

编辑 `src/lib.rs` 文件，在里面添加如下代码：

```rust
#![feature(test)]

extern crate test;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));
    }
}
```

注意：

1. 这里虽然使用了 `extern crate test;`，但是项目的 `Cargo.toml` 文件中依赖区并不需要添加对 `test` 的依赖；
2. 评测函数 `fn bench_add_two(b: &mut Bencher) {}` 上面使用 `#[bench]` 做标注，同时函数接受一个参数，`b` 就是 Rust 提供的评测器。这个写法是固定的。

然后，在工程根目录下，执行

```
cargo bench
```

输出结果类似如下：

```
$ cargo bench
   Compiling benchit v0.0.1 (file:///home/mike/tmp/benchit)
     Running target/release/benchit-91b3e234d4ed382a

running 2 tests
test tests::it_works ... ignored
test tests::bench_add_two ... bench:         1 ns/iter (+/- 0)

test result: ok. 0 passed; 0 failed; 1 ignored; 1 measured
```

可以看到，Rust 的性能测试是以纳秒 ns 为单位。

写测评代码的时候，需要注意以下一些点：

1. 只把你需要做性能测试的代码（函数）放在评测函数中；
2. 对于参与做性能测试的代码（函数），要求每次测试做同样的事情，不要做累积和改变外部状态的操作；
3. 参数性能测试的代码（函数），执行时间不要太长。太长的话，最好分成几个部分测试。这也方便找出性能瓶颈所在地方。

