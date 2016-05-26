# Rust journey

## HelloWorld
According to the traditional programming language, learning first programming language is the first program to print Hello World!
Rust created below the Hello World According to our step! program:

** The following command operation, unless otherwise specified, are run in shell. In this paper, for simple and uniform, are all examples of powershell to run under win10, all commands run in `ps:` identifier after **

- Doing create a directory and file helloworld.rs

> Ps: mkdir ~ / Doing
> Ps: cd ~ / Doing
> Ps: notepad helloworld.rs # sublime authors prefer to use as editor
> Ps: subl helloworld.rs # subl chapter later use instead of notepad

Note the use of the suffix is ​​.rs, general programming language code files have the usual suffix, such as:
    C language is .c, java is .java, python is .py etc. ** Be sure to remember Rust language customary name suffix .rs ** (although with another extension but also through rustc compilation).

- Enter the code in Rust helloworld.rs file

`` `Rust
fn main () {
    println ( "Hello World!")!;
}
`` `

- Compile helloworld.rs file

> Ps: rustc helloworld.rs
> Ps: rustc helloworld.rs -O # You can also select Optimizing Compiler

- Run the program

> Ps: next ./helloworld.exe # windows platform need to add .exe suffix
> Hello World!

No `ps:` prefix indicates the console printout.

We have written rust first executable program, print out 'hello world!', Cool, right!
But this code in the end what does that mean, as a novice you must foggy, let us look at this program:

1. The first line fn represents a defined function ** **, main is the name of this function, curly braces {} in the statement said that the content of this function.
2. The function name is ** main ** has a special purpose, that is, as the entry procedure, which means that every time the program starts running from this function.
3. The function is only a `` `println (" Hello World! ");!` ``, Where `` `println` `` Rust is a macro language built-in ** **!
This macro function is to print the text (the end of the line will change), and "Hello World!" Wrap this thing in quotes is a string ** ** that we want to print text.
4. You must have noticed `` `;` `` right in the Rust language, the semicolon `;` `for the separated statement, that statement with a semicolon as the end of the end of the general mark.

## HelloRust

- Create Project hellorust

> Ps: cargo new hellorust --bin

- Check the directory structure

> Ps: tree # win10 powershell comes with a function to view the file directory tree structure
> └─hellorust
> ---- └─src

The directory structure shown here, in the src directory hellorust Cargo.toml folders and files, and the directory is initialized to the git project

- View Cargo.toml file

> Ps: cat Cargo.toml
> [Package]
name = "hellorust"
version = "0.1."
authors = [ "YourName <YourEmail>"]
> [Dependencies]

- Main.rs file edit src directory

> Ps: subl ./src/main.rs

Project cargo created in the src directory there will be a main.rs initialization file content:
`` `Rust
fn main () {
    println ( "Hello, world!")!;
}
`` `
Now let's edit this file, the following:
`` `Rust
fn main () {
    let rust = "Rust";
    ! Println ( "! Hello, {}", rust);
}
`` `
Where `let rust =" Rust "` is binding for the variable rust "Rust",
! `Println ("! Hello, {} ", rust);` generation of rust in the value of the variable into the `" Hello, {}! "` The `` {}.

- Compile and run

> Ps: cargo build
> Ps: cargo build --release # This belongs optimizing compiler
> Ps: ./target/debug/hellorust.exe
> Ps: ./target/release/hellorust.exe # If the front is an optimizing compiler, then this run
> Ps: cargo run # compile and run together
> Ps: cargo run --release # above, except that the optimizing compiler
