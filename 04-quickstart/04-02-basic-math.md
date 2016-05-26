Simple math # 4.2

Math is a very useful feature, in reality, will be frequently used, for example: calculate the total price of goods, the student's grade point average, personal income tax and so on.

# 4.2.1 Digital

In understanding the math, let us simply look at the `` and `` digital change on a section of the HelloWorld program:

`` `Rust
fn main () {
    let x = 5;
    println ( "{}", x)!;
}
`` `

Run the program will print out the number 5 in this program:

- Let represents definition of a ** ** variable, the variable named x
- = Symbol for assignment x ** ** and the value 5
--5 Is a digital ``, Rust will automatically put it as an integer ** ** Treatment
-! Println ( "{}", x) is the formatted output statements, {} is a placeholder that represents the value of the variable x will later be embedded here, that is, to print the value of x (here an overview on the line, subsequent chapters will explain in detail)

# 4.2.2 Simple math

We can perform addition, subtraction digital operations:

`` `Rust
fn main () {
    let x = 5 + 10;
    println ( "{}", x)!;
}
`` `

The above program will output 15, which is the result of 5 + 10. Of course, we can also do variable operation, the output of the following procedure is the same:

`` `Rust
fn main () {
    let x = 5;
    let y = 10;
    let z = x + y;
    println ( "{}", z)!;
}
`` `

Apart addition, we can also do more other operations, the following is a commonly used four operations:

| Op | Symbol | Example |
| --- |: ---: | ---: |
| Plus | + | 10 + 5 |
| Save | - | 10 - 5 |
| Multiply | * | 10 * 5 |
| Except | / | 10/5 |


# 4.2.3 integer and floating point

The above example of `let x = 5;`, five will be treated as an integer, n is an integer no fractional part, for example:

`` `Rust
fn main () {
    ! Println ( "{}", 5/10);
}
`` `

The program prints out the 5/10 operation result is 0, not 0.5! ** Two integers because the result is an integer division **

If you wish to obtain more accurate results, we need to use floating-point numbers:

`` `Rust
fn main () {
    ! Println ( "{}", 5.0 / 10.0);
}
`` `

This program will output 0.5, where `5.0` and` 10.0` because a decimal point, it will be treated as a floating-point, and floating-point division can obtain more accurate results
