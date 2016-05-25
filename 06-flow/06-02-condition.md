# Conditional branches

- If
- If let
- Match

## If expression

Rust if the expression is substantially the following forms:

`` `Rust
// Form 1
if expr1 {

}

// Form 2
if expr1 {

}
else {

}

// Form 3
if expr1 {

}
else if expr2 {
    // Else if multiples,
}
else {

}

`` `

With respect to the C-based language, a distinctive feature of Rust if the expression is:

1. determine the conditions do not enclosed in parentheses;
2. It is an expression, not statements.

In view of the second point, because it is an expression, so we can write the following code:

`` `Rust
let x = 5;

let y = if x == 5 {
    10
} Else {
    15
}; // Y: i32
`` `

Or compressed into a single line:

`` `Rust
let x = 5;

let y = if x == 5 {10} else {15}; // y: i32
`` `

## If let

We often see in the code `if let` pairs, which is actually simplifies the usage of a match. Direct example to illustrate:

`` `Rust
let x = Some (5);

if let Some (y) = x {
    ! Println ( "{}", y); // output here is: 5
}

let z = if let Some (y) = x {
    y
}
else {
    0
};
// Z value of 5

`` `

The above code is equivalent to

`` `Rust
let x = Some (5);
match x {
    Some (y) => println! ( "{}", Y),
    None => ()
}

let z = match x {
    Some (y) => y,
    None => 0
};
`` `

The syntax is designed for this purpose, in the conditions when the judge directly do a pattern match, easy to write the code, make the code more compact.

## Match

Rust is not C-like `switch` keywords, but it has for pattern matching` match`, can achieve the same function, and much stronger.

match very simple to use, for example as follows:

`` `Rust
let x = 5;

match x {
    1 => {
        println! ( "one")
    },
    2 => println! ( "Two"),
    3 => println! ( "Three"),
    4 => println! ( "Four"),
    5 => println! ( "Five"),
    _ => Println! ( "Something else"),
}
`` `
Note, match is also an expression. Later match will be devoted, see chapter ** ** pattern matching.
