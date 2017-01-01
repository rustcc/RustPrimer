# Rust json处理

JSON是一种比较重要的格式，尤其是现在的web开发领域，JSON相比于传统的XML更加容易操作和减小传输。

Rust中的JSON处理依赖 cargo 中的rustc-serialize模块

###先简单的创建一个Rust项目工程

``` rust
$ cargo new json_data --bin
```

生成文件树：

```shell
vagrant@ubuntu-14:~/tmp/test/rustprimer$ tree
.
`-- json_data
    |-- Cargo.toml
    `-- src
        `-- main.rs


```

生成项目`json_data`,项目下文件介绍：

- Caogo.toml ，文件中填写一些项目的相关信息，比如版本号，联系人，项目名，文件的内容如下：

```toml
[package]
name = "json_data"
version = "0.1.0"
authors = ["wangxxx <xxxxx@qq.com>"]

[dependencies]

```

- src 中放置项目的源代码，main.rs 为项目的入口文件。

###一些必要的了解

rustc-serialize 这个是第三方的模块，需要从[cargo](https://crates.io/crates/rustc-serialize)下载。
下载很简单，只需修改一下cargo.toml文件就行了.

```toml
[package]
name = "json_data"
version = "0.1.0"
authors = ["wangxxx <xxxxx@qq.com>"]

[dependencies]
rustc-serialize = "0.3.18"

```

然后执行在当前目录执行:

```
$ cargo build
```

*注意一个问题由于国内网络访问github不稳定，这些第三方库很多托管在github上，所以可能需要修改你的
网络访问*

1. 在安装Rust之后，会在你的用户目录之下生成一个`.cargo`文件夹，进入这个文件夹
2. 在`.cargo`文件夹下，创建一个`config`文件，在文件中填写中科大软件源，可能以后会出现其他的源，先用这个
3. `config`文件内容如下

```toml
[registry]
index = "git://crates.mirrors.ustc.edu.cn/index"

```

cargo build 执行之后的提示信息

```
   Updating registry `git://crates.mirrors.ustc.edu.cn/index`
 Downloading rustc-serialize v0.3.18 (registry git://crates.mirrors.ustc.edu.cn/index)
   Compiling rustc-serialize v0.3.18 (registry git://crates.mirrors.ustc.edu.cn/index)
   Compiling json_data v0.1.0 (file:///home/vagrant/tmp/test/rustprimer/json_data)
```

再次执行tree命令:

```
.
|-- Cargo.lock
|-- Cargo.toml
|-- src
|   `-- main.rs
`-- target
    `-- debug
        |-- build
        |-- deps
        |   `-- librustc_serialize-d27006e102b906b6.rlib
        |-- examples
        |-- json_data
        `-- native

```

可以看到多了很多文件，重点关注`cargo.lock`,开打文件:

```toml
[root]
name = "json_data"
version = "0.1.0"
dependencies = [
 "rustc-serialize 0.3.18 (registry+git://crates.mirrors.ustc.edu.cn/index)",
]

[[package]]
name = "rustc-serialize"
version = "0.3.18"
source = "registry+git://crates.mirrors.ustc.edu.cn/index"

```

是关于项目编译的一些依赖信息

还有生成了target文件夹，生成了可执行文件json_data,因为main.rs中的执行结果就是打印`hello world`

```
$ cargo run

Hello, world!
```

###开始写代码
直接使用官方的 [rustc_serialize 中的例子](https://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/index.html#using-autoserialization)：

``` rust
extern crate rustc_serialize;
// 引入rustc_serialize模块
use rustc_serialize::json;

// Automatically generate `RustcDecodable` and `RustcEncodable` trait
// implementations
// 定义TestStruct
#[derive(RustcDecodable, RustcEncodable)]
pub struct TestStruct  {
    data_int: u8,
    data_str: String,
    data_vector: Vec<u8>,
}

fn main() {
    // 初始化TestStruct
    let object = TestStruct {
        data_int: 1,
        data_str: "homura".to_string(),
        data_vector: vec![2,3,4,5],
    };

    // Serialize using `json::encode`
    // 将TestStruct转意为字符串
    let encoded = json::encode(&object).unwrap();
    println!("{}",encoded);
    // Deserialize using `json::decode`
    // 将json字符串中的数据转化成TestStruct对应的数据，相当于初始化
    let decoded: TestStruct = json::decode(&encoded).unwrap();
    println!("{:?}",decoded.data_vector);
}

```

当然我们也可以在文本中作为api的返回结果使用，下来的章节中，我们将讨论这个问题
