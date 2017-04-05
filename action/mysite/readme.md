# Rust web development

Since Rust is a system-level programming language, of course, can also be used to develop for the web, ___***but I like this mortal, certainly not from scratch to write a web
Server, must rely on the existing rust web development framework to complete the web development.***___

Currently the most popular web frameworks are iron and nickel, ___***we both write a simple use of tutorials.***___

## iron

We will use the cargo package manager to install and use third party libraries. `Cargo new mysite --bin`
First, lets add Iron to our project.

Add iron dependencies in cargo.toml,
```Toml
[Dependencies]
Iron = "*"
```
The build will depend on downloading to our local machine `cargo build`

If you recieve ssl errors, then you will need to install a proper ssl development library.

From the official Iron example:

```rust
extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello World!")))
    }).http("localhost:3000").unwrap();
}
```
Then

`Cargo run`

Use curl to visit your website!

`curl localhost:3000`

`// Hello World!`

At first I found that this example was a bit nonsense... ah, I am accustomed to writing python and not really used to this style.
Simple point of view:

`Iron :: new (). Http (" localhost: 3000 "). Unwrap ()`
This is the basic definition of the server, new internal is a [rust lambda expression](https://doc.rust-lang.org/book/closures.html)
```rust
let plus_one = |x: i32| x + 1;

assert_eq!(2, plus_one(1));
```
___***How to use the specific, you can temporarily ignore, because you just know how to complete the web, because I will not. 
The Combined with the previous section of the json processing, we take a look at how the web interface to return json, of course, also rustc_serialize into cargo.toml***___

* The following code refers directly to the open source code [address](https://github.com/brson/httptest#lets-make-a-web-service-and-client-in-rust) *

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
Execute cargo run using curl to test results:

```
Curl localhost: 3000
{"Msg": "Hello, World"}
```

Of course you can achieve much more complex ___***business needs by controlling your own json.***___

___***Since there is a json, and if there is more than what routing, it would not be finished, so it is impossible***___, we need to consider how to achieve the custom routing!

Do not speak directly on the code, the same in your cargo.toml file to add the dependence on the router
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
___***This time to add the implementation of the routing and access to the client to send the data, with get, post, so now a basic api site has been completed. but...
Not all sites are api to visit, the same need html template engine and directly return to static pages. and many more***___
```
Vagrant @ ubuntu-14: ~ / tmp / test / rustprimer / mysite $ cargo build
   Compiling mysite v0.1.0 (file: /// home / vagrant / tmp / test / rustprimer / mysite)
Src / main.rs: 29: 36: 29:52 error: no method named `read_to_string` found for type` iron :: request :: Body <'_,' _> `in the current scope
Src / main.rs: 29 let payload = request.body.read_to_string ();
                                                  ^ ~~~~~~~~~~~~~~~
Src / main.rs: 29: 36: 29:52 help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, maybe add a `use` for it:
Src / main.rs: 29: 36: 29:52 help: candidate # 1: use `std :: io :: Read`
Error: aborting due to previous error
Could not compile `mysite`.
```
Compiler wrong, too bad, suggesting that there is no read_to_string this method, and then I went to the document and found a [read_to_string method](http://ironframework.io/doc/iron/request/struct.Body.html)
Look at the message

```
Src / main.rs: 29: 36: 29:52 help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, maybe add a `use` for it:
Src / main.rs: 29: 36: 29:52 help: candidate # 1: use `std :: io :: Read`
```

Let us add a `std :: io :: Read`, this if the operation of the file, you must know how to write, add it, should be able to go, or continue to go wrong, look at the error
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

The first sentence prompts us that the read_to_string (), at least one parameter, but we are not provided.
We look at the use of [read_to_string](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_to_string)

``` rust

se std::io;
use std::io::prelude::*;
use std::fs::File;

let mut f = try!(File::open("foo.txt"));
let mut buffer = String::new();

try!(f.read_to_string(&mut buffer));

```
Usage is relatively simple, we just need to modify the function:

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
Read the string from the request, read the results stored in the payload, and then you can operate, compile and run, use curl to submit a post data

```
curl -X POST -d '{"msg":"Just trust the Rust"}' http://localhost:3000/set
{"msg":"Just trust the Rust"}
```
Iron basically come to an end
Of course, how to use html template engine, that is, directly to see the document on the line.

## [nickel](http://nickel.rs/)

Of course since the web framework is definitely iron capable nicke is also capable, so then we look at how to do a hello and return a html
The page

Also we create `cargo new site - bin`, then add nickel to cargo.toml,` cargo build`

```Rust

# [Macro_use] extern crate nickel;

Use nickel :: Nickel;

Fn main () {
    Let mut server = Nickel :: new ();

    Server.utilize (router! {
        Get "**" => | _req, _res | {
            "Hello world!"
        }
    });

    Server.listen ("127.0.0.1:6767");
}
```
In simple terms, that is the case.

1. The introduction of the nickel macro
2. Initialize Nickel
3. Call the phone to define the routing module.
4.router!` Macro, the incoming parameter is the get method and the corresponding path, "\ * \ *" is the full path match.
Listen to start the server

[Of course we want to introduce information about html templates](http://nickel.rs/#easy-templating)

```Rust
# [Macro_use] extern crate nickel;

Use std :: collections :: HashMap;
Use nickel :: {Nickel, HttpRouter};

Fn main () {
    Let mut server = Nickel :: new ();

    Server.get ("/", middleware! {| _, Response |
        Let mut data = HashMap :: new ();
        Data.insert ("name", "user");
        Return response.render ("site / assets / template.tpl", & data);
    });

    Server.listen ("127.0.0.1:6767");
}

```
The above information you can compile, use curl to see the discovery

```
$ Curl http://127.0.0.1:6767
Internal Server Error
```
Look at the document, did not find any problems, I tightly changed the name of a folder, this folder I also created.
And then i wonder if the server will write the directory to die? So the above path to correct this, the problem is solved.

```Rust
Return response.render ("examples / assets / template.tpl", & data);
```
Let's take a look at the directory structure

```
The
| - Cargo.lock
| - Cargo.toml
- examples
|- assets
|- template.tpl
| - src
|- main.rs

```
