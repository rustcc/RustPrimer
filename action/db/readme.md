# rust数据库操作

编程时，我们依赖数据库来存储相应的数据，很多编程语言都支持对数据库的操作，所以当然可以使用Rust操作数据库。

不过在我自己操作时，发现很多问题，主要因为我不了解Rust在操作数据库时，应该注意的事情，从而浪费了很多的时间，在进行数据查询时。
具体遇到的坑，我会做一些演示，从而让大家避免这些情况。

首先使用Rust操作PostgreSQL,因为PostgreSQL是我最喜欢的数据库。

首先创建新项目 `cargo new db --bin`

在cargo.toml中添加 `postgres` 如下：


``` rust
[package]
name = "db"
version = "0.1.0"
authors = ["vagrant"]

[dependencies]
postgres="*"
```


当然我们还是进行最简单的操作，直接粘贴复制，[代码来源](https://github.com/sfackler/rust-postgres#overview)

``` rust

extern crate postgres;

use postgres::{Connection, SslMode};

struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}

fn main() {
    let conn = Connection::connect("postgres://postgres@localhost", SslMode::None)
            .unwrap();

    conn.execute("CREATE TABLE person (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    data            BYTEA
                  )", &[]).unwrap();
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None
    };
    conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
                 &[&me.name, &me.data]).unwrap();

    for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2)
        };
        println!("Found person {}", person.name);
    }
}

```

这些简单的，当然不是我们想要的东西，我们想要的是能够进行一些分层，也就是
基本的一些函数逻辑划分，而不是在一个main函数中，完成所有的一切。

##创建lib.rs文件

从上到下来看文件：

1. 首先导入postgres的各种库
2. 创建一个Person 的struct，按照需求的字段和类型。
3. 创建一个连接函数，返回连接对象。
4. 创建一个插入函数，用来插入数据
5. 创建一个查询函数，用来查询数据
6. 创建一个查询函数，用来查询所有的数据。

当然这些函数都是有一定的功能局限性。

``` rust

extern crate postgres;

use postgres::{Connection, SslMode};
use postgres::types::FromSql;
use postgres::Result as PgResult;


struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}


pub fn connect() -> Connection{
    let dsn = "postgresql://postgres:2015@localhost/rust_example";
    Connection::connect(dsn, SslMode::None).unwrap()
}

pub fn insert_info(conn : &Connection,title : &str, body: &str){

    let stmt = match conn.prepare("insert into blog (title, body) values ($1, $2)") {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("Preparing query failed: {:?}", e);
            return;
        }
    };
        stmt.execute(&[&title, &body]).expect("Inserting blogposts failed");
}


pub fn query<T>(conn: &Connection,query: &str) ->PgResult<T>
        where T: FromSql {
            println!("Executing query: {}", query);
            let stmt = try!(conn.prepare(query));
            let rows = try!(stmt.query(&[]));
            &rows.iter().next().unwrap();
            let row = &rows.iter().next().unwrap();
                //rows.iter().next().unwrap()
            row.get_opt(2).unwrap()

}

pub fn query_all(conn: &Connection,query: &str){
            println!("Executing query: {}", query);
            for row in &conn.query(query,&[]).unwrap(){
                let person = Person{
                    id: row.get(0),
                    name: row.get(1),
                    data: row.get(2)
            };
            println!("Found person {}", person.name);
            }

}

```

然后在main.rs 中调用相应的函数代码如下
1. extern db ,引入db，也就是将项目本身引入
2. use db 使用db，中的可以被引入的函数
3. 定义Blog,由于个人使用blog表，是自己创建，所以如果报错说不存在表，需要你自己去创建
4. 使用lib中定义的函数，进行最基本的一些操作

``` rust
extern crate postgres;
extern crate db;

use postgres::{Connection, SslMode};

use db::*;

struct Blog {
    title: String,
    body:  String,
}

fn main() {
    let conn:Connection=connect();

    let blog = Blog{
        title: String::from("title"),
        body: String::from("body"),
    };
    let title = blog.title.to_string();
    let body = blog.body.to_string();
    insert_info(&conn,&title,&body);

   for row in query::<String>(&conn,"select * from blog"){
        println!("{:?}",row);
    }
    let sql = "select * from person";
    query_all(&conn,&sql);
}

```

自己遇到的坑

- 创建连接函数时，连接必须有一个返回值，所以必须指定返回值的类型，
对于一个写Python的人而言，我觉得是痛苦的，我想按照官方的写法match
一下，发现可能产生多个返回值。在编译时直接无法通过编译，所以最终
使用了unwrap,解决问题，不过我还是没有学会，函数多值返回时我如何
定义返回值

- 在使用`&conn.query(query,&[]).unwrap()`时，我按照文档操作，文档说
返回的是一个可迭代的数据，那也就是说，我可以使用for循环，将数据打印，
但是发现怎么也不能实现：

``` rust

pub fn query_all(conn: &Connection,query: &str){
            println!("Executing query: {}", query);
            for row in &conn.query(query,&[]).unwrap(){
                  println!("Found person {:?}", row.get_opt(1));
            }
}

```

报错如下：

``` rust
vagrant@ubuntu-14:~/tmp/test/rustprimer/db$ cargo run
   Compiling db v0.1.0 (file:///home/vagrant/tmp/test/rustprimer/db)
src/lib.rs:53:37: 53:47 error: unable to infer enough type information about `_`; type annotations or generic parameter binding required [E0282]
src/lib.rs:53   println!("Found person {:?}", row.get_opt(1));
                                                  ^~~~~~~~~~
<std macros>:2:25: 2:56 note: in this expansion of format_args!
<std macros>:3:1: 3:54 note: in this expansion of print! (defined in <std macros>)
src/lib.rs:53:3: 53:49 note: in this expansion of println! (defined in <std macros>)
src/lib.rs:53:37: 53:47 help: run `rustc --explain E0282` to see a detailed explanation
error: aborting due to previous error
Could not compile `db`.

```

然后去查看了关于postgres模块的所有函数，尝试了无数种办法，依旧没有解决。

可能自己眼高手低，如果从头再把Rust的相关教程看一下，可能很早就发现这个问题，
也有可能是因为习惯了写Python，导致自己使用固有的思维来看待问题和钻牛角尖，才
导致出现这样的问题，浪费很多的时间。

- 改变思维，把自己当作一个全新的新手，既要利用已有的思想来学习新的语言，同样不要
被自己很精通的语言，固化自己的思维。

