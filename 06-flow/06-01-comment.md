Comment #

Rust code file, usually we can see three kinds of comments.

- Line comments
- Documentation Comments
- Module Notes

##-Line comments

// Use `` conduct line comment.

`` `Rust
// Create a binding
let x = 5;

let y = 6; // create another binding
`` `

## Documentation Comments

Note the use of the document `` `` `` /// described generally used for a function or structure (field), placed over the object to be described. Doc comments can be used inside markup syntax markdown format, it can be used for automatic document rustdoc extraction tool.

    /// Adds one to the number given.
    ///
    /// # Examples
    ///
    /// `` `
    /// Let five = 5;
    ///
    /// Assert_eq (6, add_one (5))!;
    /// # Fn add_one (x: i32) -> i32 {
    /// # X + 1
    } /// #
    /// `` `
    fn add_one (x: i32) -> i32 {
        x + 1
    }


## Module Notes

Note the use of the module `` `@!` `, For explaining the function of the module. Usually placed in the head module file.

`` `Rust
//! # The Rust Standard Library
//!
//! The Rust Standard Library provides the essential runtime
//! Functionality for building portable Rust software.
`` `

PS: /// respect `` @ `` used to annotate it contains items (that is to say, crate, module or function), rather than in the entry after it!.


## Other: Compatible with C language comments

Rust also supports compatible C block comments written: `/ * * /`. But not recommended, please try not to use this comment style (be despised).

`` `Rust
/ *
    let x = 42;
    println ( "{}", x)!;
* /
`` `
