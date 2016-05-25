# basic type

## Variable bindings
Rust by keyword let variable bindings.
`` `Rust
fn main () {
    let a1 = 5;
    let a2: i32 = 5;
    assert_eq (a1, a2)!;
    // Let bind default integer variable type inference is i32

    let b1: u32 = 5;
    // Assert_eq (a1, b1)!;
    // Remove the above comments will be error, because the type does not match
    // Errer: mismatched types
}
`` `
Here assert_eq! Macro is used to determine the two parameters are not equal, but if the two do not match the type of error will be equal even literal.

## Variable bindings
rust when you declare a variable, the variable is added in front of mut keywords will become variable variable variable bindings.
`` `Rust
fn main () {
    let mut a: f64 = 1.0;
    let b = 2.0f32;

    // Change a binding
    a = 2.0;
    println ( "{:?}", a)!;

    // Rebinding immutable
    let a = a;

    // Can not be assigned
    // A = 3.0;

    // Type mismatch
    // Assert_eq (a, b)!;
}
`` `
Here b variables bound 2.0f32. This is the type of value inside Rust explicit markup language issue. The syntax is defined as `value` +` tpye` form.

**E.g:**
Fixed-size type:
> 1u8 1i8
> 1u16 1i16
> 1u32 1i32
> 1u64 1i64

Variable size type:
> 1usize 1isize

Floating-point type:
> 1f32 1f64

## Let Deconstruction
Why Rust life inside a variable time to use let-binding syntax?
That is because the let binding expression syntax stronger, and let the language is actually a pattern.

**E.g:**
`` `Rust
fn main () {
    let (a, mut b): (bool, bool) = (true, false);
    ! Println ( "a = {:?}, B = {:?}", A, b);
    // A immutable binding
    // A = false;
    
    // B variable bindings
    b = true;
    assert_eq (a, b)!;
}
`` `
As used herein, a bool, only two values ​​true and false, are generally used for the type of logic judgment.
