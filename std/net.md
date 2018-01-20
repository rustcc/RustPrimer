# 网络模块:W猫的回音

本例子中，W猫将带大家写一个大家都写过但是没什么人用过的TCP ECHO软件，作为本章的结尾。本程序仅作为实例程序，我个人估计也没有人在实际的生活中去使用她。不过，作为标准库的示例来说，已经足够。

首先，我们需要一个一个服务器端。

```rust
fn server<A: ToSocketAddrs>(addr: A) -> io::Result<()> {
    // 建立一个监听程序
    let listener = try!(TcpListener::bind(&addr)) ;
    // 这个程序一次只需处理一个链接就好
    for stream in listener.incoming() {
        // 通过match再次解包 stream到
        match stream {
            // 这里匹配的重点是如何将一个mut的匹配传给一个Result
            Ok(mut st) => {
                // 我们总是要求client端先发送数据
                // 准备一个超大的缓冲区
                // 当然了，在实际的生活中我们一般会采用环形缓冲来重复利用内存。
                // 这里仅作演示，是一种很低效的做法
                let mut buf: Vec<u8> = vec![0u8; 1024];
                // 通过try!方法来解包
                // try!方法的重点是需要有特定的Error类型与之配合
                let rcount = try!(st.read(&mut buf));
                // 只输出缓冲区里读取到的内容
                println!("{:?}", &buf[0..rcount]);
                // 回写内容
                let wcount = try!(st.write(&buf[0..rcount]));
                // 以下代码实际上算是逻辑处理
                // 并非标准库的一部分了
                if rcount != wcount {
                    panic!("Not Fully Echo!, r={}, w={}", rcount, wcount);
                }
                // 清除掉已经读到的内容
                buf.clear();
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
    // 关闭掉Serve端的链接
    drop(listener);
    Ok(())
}

```


然后，我们准备一个模拟TCP短链接的客户端：

```rust
fn client<A: ToSocketAddrs>(addr: A) -> io::Result<()> {

    let mut buf = vec![0u8;1024];
    loop {
        // 对比Listener，TcpStream就简单很多了
        // 本次模拟的是tcp短链接的过程，可以看作是一个典型的HTTP交互的基础IO模拟
        // 当然，这个通讯里面并没有HTTP协议 XD！
        let mut stream = TcpStream::connect(&addr).unwrap();
        let msg = "WaySLOG comming!".as_bytes();
        // 避免发送数据太快而刷屏
        thread::sleep_ms(100);
        let rcount = try!(stream.write(&msg));
        let _ = try!(stream.read(&mut buf));
        println!("{:?}", &buf[0..rcount]);
        buf.clear();
    }
    Ok(())
}

```

将我们的程序拼接起来如下：

```rust
use std::net::*;
use std::io;
use std::io::{Read, Write};
use std::env;
use std::thread;

fn server<A: ToSocketAddrs>(addr: A) -> io::Result<()> { .. }


fn client<A: ToSocketAddrs>(addr: A) -> io::Result<()> { .. }


fn main() {
    let mut args = env::args();
    args.next();
    let action = args.next().unwrap();
    if action == "s" {
        server(&args.next().unwrap()).unwrap();
    } else {
        client(&args.next().unwrap()).unwrap();
    }
}

```

各位可以自己试一下结果


写网络程序，注定了要处理各种神奇的条件和错误，定义自己的数据结构，粘包问题等都是需要我们去处理和关注的。相较而言，Rust本身在网络方面的基础设施建设并不尽如人意，甚至连网络I/O都只提供了如上的block I/O 。可能其团队更关注于语言基础语法特性和编译的改进，但其实，有着官方出品的这种网络库是非常重要的。同时，我也希望Rust能够涌现出更多的网络库方案，让Rust的明天更好更光明。
