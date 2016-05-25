# 4.3 Quick Start - Quick Reference Guide

> Following from this section [learnxinyminutes] (https://github.com/adambard/learnxinyminutes-docs), the author is [Guangming Mao] (http://maogm.com/), to comply with creative commons agreement.

Rust is a programming language developed by Mozilla Institute. Rust will facilitate control of the underlying performance and security high-level language and combine together.

The Rust does not need to achieve this goal a garbage collector or run time, which makes Rust library can become a substitute for the C language.

Rust First Edition (version 0.1) was released in January 2012, he has been in full swing in iteration 3 years.
Because the update too frequently, usually recommend building stable release version instead use per night, until recently released version 1.0.

March 15, 2015, Rust 1.0 release, perfect backward compatibility, the latest nightly build version provides new features such as compilation time.
Rust uses a continuous iterative model, a release every six weeks. Rust 1.1 beta version 1.0 release also released.

Although Rust is a relatively low-level language, it provides a common language for advanced functional programming features. This allows Rust is not only efficient, and easy to use.

`` `Rust
// This is a comment, single-line comments ...
/ * ... This is a multi-line comment * /

///////////////
// // 1. Basic
///////////////

// Function (Functions)
// `I32` is a signed 32-bit integer type (32-bit signed integers)
fn add2 (x: i32, y: i32) -> i32 {
    // Implicit return (not a semicolon)
    x + y
}

// Main function (Main function)
fn main () {
    // Digital (Numbers) //

    // Immutable binding
    let x: i32 = 1;

    // Plastic / floating-point number suffix
    let y: i32 = 13i32;
    let f: f64 = 1.3f64;

    // Type inference
    // Most of the time, Rust compiler deduced types of variables, it is not necessary to type explicitly written.
    // This tutorial there are many places to explicitly write types, but only for demonstration.
    // Type inference can be handed most of the time.
    let implicit_x = 1;
    let implicit_f = 1.3;

    // Arithmetic
    let sum = x + y + 13;

    // Variable Variable
    let mut mutable = 1;
    mutable = 4;
    mutable + = 2;

    // String (Strings) //

    // String literal
    let x: & str = "hello world!";

    // Output
    println ( "{} {}", f, x);! // 1.3 hello world

    // A `String` - string space allocated on the heap
    let s: String = "hello world" .to_string ();

    // String slicing (slice) - immutable view of another string
    // Basically immutable pointer pointing to a string, it does not contain any of the contents of the string, just a pointer pointing to something
    // For example, this is `s`
    let s_slice: & str = & s;

    ! Println ( "{} {}", s, s_slice); // hello world hello world

    // Array (Vectors / arrays) //

    // Fixed length arrays (array)
    let four_ints: [i32; 4] = [1, 2, 3, 4];

    // Variable length arrays (vector)
    let mut vector: Vec <i32> = vec [1, 2, 3, 4];!
    vector.push (5);

    // Fragmentation - an array (vector / array) immutable view
    // String slicing and essentially the same, except for array
    let slice: & [i32] = & vector;

    `{// Use:?}` Press debugging output style
    ! Println ( "{:?} {:?}", Vector, slice); // [1, 2, 3, 4, 5] [1, 2, 3, 4, 5]

    // Tuple (Tuples) //

    // Tuple is a set of values ​​fixed size, can be of different types
    let x: (i32, & str, f64) = (1, "hello", 3.4);

    // Deconstruction `let`
    let (a, b, c) = x;
    println ( "{} {} {}", a, b, c);! // 1 hello 3.4

    // Index
    ! Println ( "{}", x.1); // hello

    //////////////
    // 2. Type (Type) //
    //////////////

    // Structure (Sturct)
    struct Point {
        x: i32,
        y: i32,
    }

    let origin: Point = Point {x: 0, y: 0};

    // Anonymous member of a structure, called "tuples structure" ( 'tuple struct')
    struct Point2 (i32, i32);

    let origin2 = Point2 (0, 0);

    // C style based on enumerated types (enum)
    enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    let up = Direction :: Up;

    There are members of the enumeration type //
    enum OptionalI32 {
        AnI32 (i32),
        Nothing,
    }

    let two: OptionalI32 = OptionalI32 :: AnI32 (2);
    let nothing = OptionalI32 :: Nothing;

    // Generics (Generics) //

    struct Foo <T> {bar: T}

    // This has achieved the standard library inside, called `Option`
    enum Optional <T> {
        SomeVal (T),
        NoVal,
    }

    // Method (Methods) //

    impl <T> Foo <T> {
        // Method requires an explicit `self` parameters
        fn get_bar (self) -> T {
            self.bar
        }
    }

    let a_foo = Foo {bar: 1};
    println ( "{}", a_foo.get_bar ());! // 1

    // Interface (Traits) (or any other language, called interfaces typeclasses) //

    trait Frobnicate <T> {
        fn frobnicate (self) -> Option <T>;
    }

    impl <T> Frobnicate <T> for Foo <T> {
        fn frobnicate (self) -> Option <T> {
            Some (self.bar)
        }
    }

    let another_foo = Foo {bar: 1};
    ! Println ( "{:?}", another_foo.frobnicate ()); // Some (1)

    ///////////////////////////////////
    // 3. Pattern Matching (Pattern matching) //
    ///////////////////////////////////

    let foo = OptionalI32 :: AnI32 (1);
    match foo {
        ! OptionalI32 :: AnI32 (n) => println ( "it's an i32: {}", n),
        OptionalI32 :: Nothing => println! ( "It's nothing!"),
    }

    // Advanced Pattern Matching
    struct FooBar {x: i32, y: OptionalI32}
    let bar = FooBar {x: 15, y: OptionalI32 :: AnI32 (32)};

    match bar {
        FooBar {x: 0, y: OptionalI32 :: AnI32 (0)} =>
            println! ( "The numbers are zero!"),
        FooBar {x: n, y: OptionalI32 :: AnI32 (m)} if n == m =>
            println! ( "The numbers are the same"),
        FooBar {x: n, y: OptionalI32 :: AnI32 (m)} =>
            println! ( "Different numbers: {} {}", n, m),
        FooBar {x: _, y: OptionalI32 :: Nothing} =>
            println! ( "The second number is Nothing!"),
    }

    ///////////////////////////////
    // 4. Process Control (Control flow) //
    ///////////////////////////////

    // `For` cycle
    let array = [1, 2, 3];
    for i in array.iter () {
        println ( "{}", i)!;
    }

    // Interval (Ranges)
    for i in 0u32..10 {
        print ( "{}", i)!;
    }
    ! Println ( "");
    // Output `0123456789 '

    // `If`
    if 1 == 1 {
        ! Println ( "Maths is working!");
    } Else {
        println ( "Oh no ...")!;
    }

    // `If` can When the expression
    let value = if true {
        "Good"
    } Else {
        "Bad"
    };

    // `While` cycle
    while 1 == 1 {
        ! Println ( "The universe is operating normally.");
    }

    // Infinite loop
    loop {
        println ( "Hello!")!;
    }

    ////////////////////////////////////////////////
    // 5. Memory Security and pointer (Memory safety & pointers) //
    ////////////////////////////////////////////////

    // Exclusive pointer (Owned pointer) - the same time, only one object can "own" the pointer
    // Means `Box` after leaving his scope, can be safely released
    let mut mine: Box <i32> = Box :: new (3);
    * Mine = 5; // dereference
    // `Now_its_mine` acquired ownership of` mine`. In other words, `mine` move (move) the
    let mut now_its_mine = mine;
    * Now_its_mine + = 2;

    println ( "{}", now_its_mine);! // 7
    // Println ( "{}", mine);! // Compiler error, because now the exclusive `now_its_mine` pointer

    // Reference (Reference) - a reference to an immutable pointers to other data
    // When the reference point to a certain value, which we call "borrow" this value, because borrowing is immutable, it can not be changed, can not move
    // Borrowing continued until the end of the life cycle, that is out of scope
    let mut var = 4;
    var = 3;
    let ref_var: & i32 = & var;

    println ( "{}", var);! // unlike `box`,` var` can continue to use
    println ( "{}", * ref_var)!;
    // Var = 5; // compiler error, because `var` be borrowed
    // * Ref_var = 6; // compiler error, because `ref_var` are immutable references

    // Variable references (Mutable reference)
    // When a variable is variably borrow, and do not use
    let mut var2 = 4;
    let ref_var2: & mut i32 = & mut var2;
    * Ref_var2 + = 2;

    println ( "{}", * ref_var2);! // 6
    // Var2 = 2; // compiler error, because `var2` be borrowed
}
`` `

## More in-depth information

Rust and many other things - it's just Rust basic features to help you understand Rust inside the most important thing.
If you want in-depth study Rust, can be read
[The Rust Programming Language] (http://doc.rust-lang.org/book/index.html)
Or on reddit [/r/rust](http://reddit.com/r/rust) subscription.
While small partners #rust channel on irc.mozilla.org also very welcome new friends.

Rust some of the features you can try on this online compiler [Rust playpen] (http://play.rust-lang.org)
Or on the [official website] (http://rust-lang.org).
