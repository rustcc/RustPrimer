# 4.1 The first Rust program - Hello, World!

> The only way to learn a new programming language is to use it to write programs.
> For all beginners, the first program written in almost all the same, namely:
>
> Print * hello, world *
>
> - Brian Kernighan "The C Programming Language"

## 4.1.1 The first program Rust

Rust's HelloWorld program and some other languages ​​are very similar, very clear and concise.

1. First, create a file called 'hello.rs' files.

    > Note the use of the suffix is ​​.rs, general programming language code files have the usual suffix, such as:
    C language is .c, java is .java, python is .py etc. ** Be sure to remember Rust language customary name suffix .rs ** (although with another extension but also through rustc compilation).

2. Use your favorite editor hello.rs, enter the following code

`` `Rust
fn main () {
    println ( "hello world!")!;
}
`` `

3. Save the above code, we find hello.rs just a text file that contains the code, and can not run.
We need to put a text file that contains rust conversion code into an executable file, a process called ** ** compile, we can through the following command to compile the code

`` `
rustc hello.rs
`` `

4. If the program there is nothing wrong (such as missing or misspelled characters), our program will compile successfully, generate an executable file hello, enter the following command to run it

`` `
./hello
`` `

5. You can see the results, it will print out:

`` `
hello world!
`` `

Great, this may be your first use Rust write a program, but it really is a complete program.

## 4.1.2 Simple explanation

We have written rust first executable program, print out 'hello world!', Cool, right!
But this code in the end what does that mean, as a novice you must foggy, let us look at this program:

- Fn represents the first line defines a function ** **, main is the name of this function, curly braces {} in the statement said that the content of this function
- The function name is ** main ** has a special purpose, that is, as the entry procedure, which means that every time the program starts running from this function
- Only one function `` `println (" hello world! ");` ``, Where `` `println` `` Rust is a macro language built-in ** **!!
This macro function is to print the text (the end of the line will change), and the "hello world!" Wrap this thing in quotes is a string ** ** that we want to print text friends
- You must have noticed `` `;` `` right in the Rust language, the semicolon `;` `for the separated statement, that statement at the end with a semicolon as a general end flag

You can try to put quotes replaced by another version of hello world!:

`` `Rust
fn main () {
    ! Println ( "Hello, World!");
}
`` `

This program will print the Chinese `` `Hello, world!` `` Is not very surprised, because the Rust language utf8 encoding is used by default, so you can deal with Chinese string!
