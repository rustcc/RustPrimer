# Loop

- For
- While
- Loop
- Break and continue
- Label


## For

for statement to traverse an iterator.
`` `Rust
for var in iterator {
    code
}
`` `
Rust iterator returns a series of elements, each cycle is one repetition. Then its value var bindings, which are valid in loop body. Whenever the loop after the implementation, we have taken the next value from the iterator, and then we repeat. When the iterator no value, for end of the cycle.

such as:
`` `Rust
for x in 0..10 {
    ! Println ( "{}", x); // x: i32
}
`` `
Export
`` `
0
1
2
3
4
5
6
7
8
9
`` `

Not familiar with the concept of iterator students may dumbfounded, not imitation the following form for the statement in C to do the next comparison:

`` `Rust
for example loop // C language
for (x = 0; x <10; x ++) {
    printf ( "% d \ n", x);
}
`` `

Both the output is the same, then why Rust should be designed for such statements do?

1. Simplified determine the boundary conditions and reduce errors;
2. Reduce runtime bounds checking, to improve performance.

Even for an experienced C language developers who want to manually control each element to be recycled are also complex and error-prone.

for statement is the iterator traversal of syntactic sugar.

It said iterator form is good, but it seems in the cycle, the less the index information. Rust takes into account that, when you need to record you've cycled many times, you can use the `.enumerate ()` function. such as:

`` `Rust
for (i, j) in (5..10) .enumerate () {
    ! Println ( "i = {} and j = {}", i, j);
}
`` `
Output:

`` `
i = 0 and j = 5
i = 1 and j = 6
i = 2 and j = 7
i = 3 and j = 8
i = 4 and j = 9
`` `
Another example:

`` `Rust
let lines = "Content of line one \ nContent of line two \ nContent of line three \ nContent of line four" .lines ();
for (linenumber, line) in lines.enumerate () {
    println ( "{}: {}", linenumber, line);!
}
`` `
Output:
`` `
0: Content of line one
1: Content of line two
2: Content of line three
3: Content of line four
`` `

Knowledge about iterator, iterator ** ** See chapter.

## While

Rust provided while statement, the conditional expression is true, the statement is executed body. When you are not sure how many times the cycle should be selected while.

`` `Rust
while expression {
    code
}
`` `

such as:

`` `Rust
let mut x = 5; // mut x: i32
let mut done = false; // mut done: bool

while! done {
    x + = x - 3;

    println ( "{}", x)!;

    if x% 5 == 0 {
        done = true;
    }
}
`` `

## Loop

In one case, we often encounter, it is to write an infinite loop:

`` `Rust
while true {
    // Do something
}
`` `

In view of this situation, Rust optimized to provide a statement loop.

`` `Rust
loop {
    // Do something
}
`` `

Static analysis of the main differences between `loop` and` while true` in the compilation phase.

For example, the following code:

`` `Rust
let mut a;
loop {
     a = 1;
     // ... Break ...
}
do_something (a)
`` `

If it is `loop` loop, the compiler will correctly analyze the variable` a` is initialized correctly, and if replaced by `while true`, a compilation error occurs. Analysis of the minor differences will also affect the life cycle.

## Break and continue

Similar to the C language, Rust also provides break and continue two keywords are used to control flow of the loop.

- Break out of the loop for the current layer;
- Continue to execute the next iteration of the current layer.

Like that while the above example:

`` `Rust
let mut x = 5;
let mut done = false;

while! done {
    x + = x - 3;

    println ( "{}", x)!;

    if x% 5 == 0 {
        done = true;
    }
}
`` `
It can be optimized to:

`` `Rust
let mut x = 5;

loop {
    x + = x - 3;

    println ( "{}", x)!;

    if x% 5 == 0 {break;}
}
`` `
I feel a little more intuitive.

The following example demonstrates continue usage:

`` `Rust
for x in 0..10 {
    if x% 2 == 0 {continue;}

    println ( "{}", x)!;
}
`` `
Its role is to print out the 0 ~ 9 `` odd. The results are as follows:

`` `
1
3
5
7
9
`` `

## Label

You may encounter such a situation, when you have nested loops and you want to specify which one break or continue the work. Like most languages, the default action will break or continue on the cycle of the current layer. When you want a break or continue acting on an outer loop, you can use your labels to specify the loop break or continue statement effect.

The following code will print them in the x and y are odd:

`` `Rust
'Outer: for x in 0..10 {
    'Inner: for y in 0..10 {
        if x% 2 == 0 {continue 'outer;} // continues the loop over x
        if y% 2 == 0 {continue 'inner;} // continues the loop over y
        ! Println ( "x: {}, y: {}", x, y);
    }
}
`` `
