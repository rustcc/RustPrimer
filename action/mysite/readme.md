# rust web 开发

rust既然是系统级的编程语言，所以当然也能用来开发 web,不过想我这样凡夫俗子，肯定不能从头自己写一个 web
服务器，肯定要依赖已经存在的 rust web开发框架来完成 web 开发。

rust目前比较有名的框架是iron和nickel，我们两个都写一下简单的使用教程。

##iron

接上一篇，使用cargo获取第三方库。`cargo new mysite --bin`

在cargo.toml中添加iron的依赖，

```toml
[dependencies]
iron = "*"
```

然后build将依赖下载到本地 `cargo build`

如果报ssl错误，那可能你需要安装linux的ssl开发库。

首先还是从 hello world 开始吧,继续抄袭官方的例子：

``` rust
extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello World!")))
    }).http("localhost:3000").unwrap();
}
```

然后运行

`cargo run`

使用curl直接就可以访问你的网站了。

`curl localhost:3000`

`Hello World!`

仔细一看，发现这个例子很无厘头啊，对于习惯了写python的我来说，确实不习惯。
简单点看：

`iron::new().http("localhost:3000").unwrap()`
这句是服务器的基本的定义，new内部是一个[rust lambda 表达式](https://doc.rust-lang.org/book/closures.html)

```rust
let plus_one = |x: i32| x + 1;

assert_eq!(2, plus_one(1));
```

具体的怎么使用 ，可以暂时不用理会，因为你只要知道如何完成web，因为我也不会。。
结合之前一章节的json处理，我们来看看web接口怎么返回json,当然也要 rustc_serialize 放到 cargo.toml 中

*下面的代码直接参考开源代码[地址](https://github.com/brson/httptest#lets-make-a-web-service-and-client-in-rust)*

```rust
extern crate iron;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use rustc_serialize::json;

#[derive(RustcEncodable)]
struct Greeting {
    msg: String
}

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let greeting = Greeting { msg: "Hello, World".to_string() };
        let payload = json::encode(&greeting).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    Iron::new(hello_world).http("localhost:3000").unwrap();
    println!("On 3000");
}
```

执行 cargo run 使用 curl 测试结果:

```
curl localhost:3000
{"msg":"Hello, World"}
```

当然可以可以实现更多的业务需求，通过控制自己的json。

既然有了json了，如果要多个路由什么的，岂不是完蛋了，所以不可能这样的，我们需要考虑一下怎么实现路由的定制

不说话直接上代码，同一样要在你的cargo.toml文件中添加对router的依赖

``` rust
extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;

#[derive(RustcEncodable, RustcDecodable)]
struct Greeting {
    msg: String
}

fn main() {
    let mut router = Router::new();

    router.get("/", hello_world);
    router.post("/set", set_greeting);

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let greeting = Greeting { msg: "Hello, World".to_string() };
        let payload = json::encode(&greeting).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    // Receive a message by POST and play it back.
    fn set_greeting(request: &mut Request) -> IronResult<Response> {
        let payload = request.body.read_to_string();
        let request: Greeting = json::decode(payload).unwrap();
        let greeting = Greeting { msg: request.msg };
        let payload = json::encode(&greeting).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    Iron::new(router).http("localhost:3000").unwrap();
}
```

这次添加了路由的实现和获取客户端发送过来的数据，有了get，post,所以现在一个基本的api网站已经完成了。不过
并不是所有的网站都是api来访问，同样需要html模版引擎和直接返回静态页面。等等

```
vagrant@ubuntu-14:~/tmp/test/rustprimer/mysite$ cargo build
   Compiling mysite v0.1.0 (file:///home/vagrant/tmp/test/rustprimer/mysite)
src/main.rs:29:36: 29:52 error: no method named `read_to_string` found for type `iron::request::Body<'_, '_>` in the current scope
src/main.rs:29         let payload = request.body.read_to_string();
                                                  ^~~~~~~~~~~~~~~~
src/main.rs:29:36: 29:52 help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it:
src/main.rs:29:36: 29:52 help: candidate #1: use `std::io::Read`
error: aborting due to previous error
Could not compile `mysite`.
```

编译出错了，太糟糕了，提示说没有read_to_string这个方法，然后我去文档查了一下，发现有[read_to_string方法](http://ironframework.io/doc/iron/request/struct.Body.html)
再看提示信息

```
src/main.rs:29:36: 29:52 help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it:
src/main.rs:29:36: 29:52 help: candidate #1: use `std::io::Read`
```

让我们添加一个`std::io::Read`,这个如果操作过文件，你一定知道怎么写，添加一下，应该能过去了，还是继续出错了，看看报错

```
   Compiling mysite v0.1.0 (file:///home/vagrant/tmp/test/rustprimer/mysite)
src/main.rs:30:36: 30:52 error: this function takes 1 parameter but 0 parameters were supplied [E0061]
src/main.rs:30         let payload = request.body.read_to_string();
                                                  ^~~~~~~~~~~~~~~~
src/main.rs:30:36: 30:52 help: run `rustc --explain E0061` to see a detailed explanation
src/main.rs:31:46: 31:53 error: mismatched types:
 expected `&str`,
    found `core::result::Result<usize, std::io::error::Error>`
(expected &-ptr,
    found enum `core::result::Result`) [E0308]
src/main.rs:31         let request: Greeting = json::decode(payload).unwrap();
                                                            ^~~~~~~
src/main.rs:31:46: 31:53 help: run `rustc --explain E0308` to see a detailed explanation
src/main.rs:30:36: 30:52 error: cannot infer an appropriate lifetime for lifetime parameter `'b` due to conflicting requirements [E0495]
src/main.rs:30         let payload = request.body.read_to_string();
                                                  ^~~~~~~~~~~~~~~~
src/main.rs:29:5: 35:6 help: consider using an explicit lifetime parameter as shown: fn set_greeting<'a>(request: &mut Request<'a, 'a>) -> IronResult<Response>
src/main.rs:29     fn set_greeting(request: &mut Request) -> IronResult<Response> {
src/main.rs:30         let payload = request.body.read_to_string();
src/main.rs:31         let request: Greeting = json::decode(payload).unwrap();
src/main.rs:32         let greeting = Greeting { msg: request.msg };
src/main.rs:33         let payload = json::encode(&greeting).unwrap();
src/main.rs:34         Ok(Response::with((status::Ok, payload)))
               ...
error: aborting due to 3 previous errors
Could not compile `mysite`.

```

第一句提示我们，这个read_to_string(),至少要有一个参数，但是我们一个都没有提供。
我们看看[read_to_string的用法](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_to_string)

``` rust

se std::io;
use std::io::prelude::*;
use std::fs::File;

let mut f = try!(File::open("foo.txt"));
let mut buffer = String::new();

try!(f.read_to_string(&mut buffer));

```

用法比较简单，我们修改一下刚刚的函数：

```
fn set_greeting(request: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload);
        let request: Greeting = json::decode(&payload).unwrap();
        let greeting = Greeting { msg: request.msg };
        let payload = json::encode(&greeting).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }
```

从request中读取字符串，读取的结果存放到payload中，然后就可以进行操作了，编译之后运行，使用curl提交一个post数据

```
$curl -X POST -d '{"msg":"Just trust the Rust"}' http://localhost:3000/set
{"msg":"Just trust the Rust"}
```

iron 基本告一段落
当然还有如何使用html模版引擎，那就是直接看文档就行了。

##[nickel](http://nickel.rs/)

当然既然是web框架肯定是iron能干的nicke也能干，所以那我们就看看如何做一个hello 和返回一个html
的页面

同样我们创建`cargo new site --bin`，然后添加nickel到cargo.toml中,`cargo build`

``` rust

#[macro_use] extern crate nickel;

use nickel::Nickel;

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            "Hello world!"
        }
    });

    server.listen("127.0.0.1:6767");
}
```

简单来看，也就是这样回事。

1. 引入了nickel的宏
2. 初始化Nickel
3. 调用utilize来定义路由模块。
4. `router!` 宏，传入的参数是 get 方法和对应的路径，"\*\*"是全路径匹配。
5. listen启动服务器

[当然我们要引入关于html模版相关的信息](http://nickel.rs/#easy-templating)

``` rust
#[macro_use] extern crate nickel;

use std::collections::HashMap;
use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();

    server.get("/", middleware! { |_, response|
        let mut data = HashMap::new();
        data.insert("name", "user");
        return response.render("site/assets/template.tpl", &data);
    });

    server.listen("127.0.0.1:6767");
}

```

上面的信息你可以编译，使用curl看看发现出现

```
$ curl http://127.0.0.1:6767
Internal Server Error
```

看看文档，没发现什么问题，我紧紧更换了一个文件夹的名字，这个文件夹我也创建了。
然后我在想难道是服务器将目录写死了吗？于是将上面的路径改正这个，问题解决。

```rust
return response.render("examples/assets/template.tpl", &data);
```

我们看一下目录结构

```
.
|-- Cargo.lock
|-- Cargo.toml
|-- examples
|   `-- assets
|       `-- template.tpl
|-- src
|   `-- main.rs

```
