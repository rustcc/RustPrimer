# 系统命令:调用grep

我们知道，Linux系统中有一个命令叫grep，他能对目标文件进行分析并查找相应字符串，并该字符串所在行输出。
今天，我们先来写一个Rust程序，来调用一下这个 grep 命令

```rust
use std::process::*;
use std::env::args;

// 实现调用grep命令搜索文件
fn main() {
    let mut arg_iter = args();
    // panic if there is no one
    arg_iter.next().unwrap();
    let pattern = arg_iter.next().unwrap_or("main".to_string());
    let pt =  arg_iter.next().unwrap_or("./".to_string());
    let output = Command::new("/usr/bin/grep")
        .arg("-n")
        .arg("-r")
        .arg(&pattern)
        .arg(&pt)
        .output()
        .unwrap_or_else(|e| panic!("wg panic because:{}", e));
    println!("output:");
    let st = String::from_utf8_lossy(&output.stdout);
    let lines = st.split("\n");
    for line in lines {
        println!("{}", line);
    }
}

```

看起来好像还不错，但是，以上的程序有一个比较致命的缺点——因为Output是同步的，因此，一旦调用的目录下有巨大的文件，grep的分析将占用巨量的时间。这对于一个高可用的程序来说是不被允许的。

那么如何改进呢？

其实在上面的代码中，我们隐藏了一个 `Child` 的概念，即——子进程。

下面我来演示怎么操作子进程：

```rust
use std::process::*;
use std::env::args;

// 实现调用grep命令搜索文件
fn main() {
    let mut arg_iter = args();
    // panic if there is no one
    arg_iter.next();
    let pattern = arg_iter.next().unwrap_or("main".to_string());
    let pt =  arg_iter.next().unwrap_or("./".to_string());
    let child = Command::new("grep")
        .arg("-n")
        .arg("-r")
        .arg(&pattern)
        .arg(&pt)
        .spawn().unwrap();
    // 做些其他的事情
    std::thread::sleep_ms(1000);
    println!("{}", "计算很费时间……");
    let out = child.wait_with_output().unwrap();
    let out_str = String::from_utf8_lossy(&out.stdout);
    for line in out_str.split("\n") {
        println!("{}", line);
    }
}

```

但是，这个例子和我们预期的并不太一样！

```
./demo main /home/wayslog/rust/demo/src
/home/wayslog/rust/demo/src/main.rs:5:fn main() {
/home/wayslog/rust/demo/src/main.rs:9:    let pattern = arg_iter.next().unwrap_or("main".to_string());
计算很费时间……

```

为什么呢？

很简单，我们知道，在Linux中，`fork`出来的函数会继承父进程的所有句柄。因此，子进程也就会继承父进程的标准输出，也就是造成了这样的问题。这也是最后我们用out无法接收到最后的输出也就知道了，因为在前面已经被输出出来了呀！

那么怎么做呢？给这个子进程一个pipeline就好了！

```rust
use std::process::*;
use std::env::args;

// 实现调用grep命令搜索文件
fn main() {
    let mut arg_iter = args();
    // panic if there is no one
    arg_iter.next();
    let pattern = arg_iter.next().unwrap_or("main".to_string());
    let pt =  arg_iter.next().unwrap_or("./".to_string());
    let child = Command::new("grep")
        .arg("-n")
        .arg("-r")
        .arg(&pattern)
        .arg(&pt)
        // 设置pipeline
        .stdout(Stdio::piped())
        .spawn().unwrap();
    // 做些其他的事情
    std::thread::sleep_ms(1000);
    println!("{}", "计算很费时间……");
    let out = child.wait_with_output().unwrap();
    let out_str = String::from_utf8_lossy(&out.stdout);
    for line in out_str.split("\n") {
        println!("{}", line);
    }
}
```

这段代码相当于给了`stdout`一个缓冲区，这个缓冲区直到我们计算完成之后才被读取，因此就不会造成乱序输出的问题了。

这边需要注意的一点是，一旦你开启了一个子进程，那么，无论你程序是怎么处理的，最后一定要记得对这个`child`调用`wait`或者`wait_with_output`，除非你显式地调用`kill`。因为如果父进程不`wait`它的话，它将会变成一个僵尸进程！！！

*注*： 以上问题为Linux下Python多进程的日常问题，已经见怪不怪了。
