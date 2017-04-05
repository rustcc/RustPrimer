# Closes as a parameter and a return value
## closures as a parameter (Taking closures as arguments)

Now we know that the closures are trait, we already know how to accept and return the closures; just like any other trait!

This also means that we can also choose static or dynamic distribution. First, let's write a function that takes a callable structure, call it, and then return the result:

`` `Rust
Fn call_with_one <F> (some_closure: F) -> i32
    Where F: Fn (i32) -> i32 {

    Some_closure (1)
}

Let answer = call_with_one (| x | x + 2);

Assert_eq! (3, answer);
`` ``

We pass our closure, `| x | x + 2`, to` call_with_one`. It is doing what we say: it calls the closure, `1` as a parameter.

Let's drill down on the signature of `call_with_one`:

`` `Rust
Fn call_with_one <F> (some_closure: F) -> i32
# Where F: Fn (i32) -> i32 {
# Some_closure (1)}
`` ``

We get a parameter, and it has the type `f`. We also return a `i32`. This part is not interesting. The next section is:

`` `Rust
# Fn call_with_one <F> (some_closure: F) -> i32
    Where F: Fn (i32) -> i32 {
# Some_closure (1)}
`` ``

Because `Fn` is a trait, we can use it to limit our generics. In this example, our closure gets a `i32` as a parameter and returns` i32`, so we use the generic limit to be `Fn (i32) -> i32`.

There is also a key point: because we use a trait to limit the generic, it will be singleton, and therefore, we use static distribution in the closure. This is very simple. In many languages, the closures are fixed on the heap, so they are always distributed dynamically. In Rust, we can allocate our closed environment on the stack and save the calls statically. This often happens on iterators and their adapters, and they often get closures as arguments.

Of course, if we want to be dynamically distributed, we can do it. Trait object handling this situation, usually:

`` `Rust
Fn call_with_one (some_closure: & Fn (i32) -> i32) -> i32 {
    Some_closure (1)
}

Let answer = call_with_one (& | x | x + 2);

Assert_eq! (3, answer);
`` ``

Now we get a trait object, a `& Fn`. And when we pass our closure to `call_with_one` we have to get a reference, so we use` & || `.

## Function pointers and closures

A function pointer is a bit like an environment without a closure. So you can pass a function pointer to any function in addition to the closing parameters, the following code can work:

`` `Rust
Fn call_with_one (some_closure: & Fn (i32) -> i32) -> i32 {
    Some_closure (1)
}

Fn add_one (i: i32) -> i32 {
    I + 1
}

Let f = add_one;

Let answer = call_with_one (& f);

Assert_eq! (2, answer);
`` ``

In this example, we are not strictly in need of this intermediate variable `f`, the name of the function:

`` `Rust
Let answer = call_with_one (& add_one);
`` ``

## Returning closures

It is very common for a functional style code to return a closure in a variety of situations. If you try to return a closure, you may get a mistake. In the first contact, this looks a bit strange, but we will find out. When you try to return a closure from the function, you might write something like this:

`` `Rust
Fn factory () -> (Fn (i32) -> i32) {
    Let num = 5;

    X | x + num
}

Let f = factory ();

Let answer = f (1);
Assert_eq! (6, answer);
`` ``

Compiler will give this long string of related errors:

`` `Text
Error: the trait `core :: marker :: Sized` is not implemented for the type
`Core :: ops :: Fn (i32) -> i32` [E0277]
Fn factory () -> (Fn (i32) -> i32) {
                ^ ~~~~~~~~~~~~~~~
Note: `core :: ops :: Fn (i32) -> i32` does not have a constant size known at compile-time
Fn factory () -> (Fn (i32) -> i32) {
                ^ ~~~~~~~~~~~~~~~
Error: the trait `core :: marker :: Sized` is not implemented for the type` core :: ops :: Fn (i32) -> i32` [E0277]
Let f = factory ();
    ^
Note: `core :: ops :: Fn (i32) -> i32` does not have a constant size known at compile-time
Let f = factory ();
    ^
`` ``

In order to return something from the function, Rust needs to know the size of the return type. But `Fn` is a trait, it can be anything of size. For example, the return value can be any type that implements `Fn`. A simple solution is to return a reference. Because the size of the reference is fixed, the size of the return value is fixed. So we can write like this:

`` `Rust
Fn factory () -> & (Fn (i32) -> i32) {
    Let num = 5;

    X | x + num
}

Let f = factory ();

Let answer = f (1);
Assert_eq! (6, answer);
`` ``

But this will be another mistake:

`` `Text
Error: missing lifetime specification [E0106]
Fn factory () -> & (Fn (i32) -> i32) {
                ^ ~~~~~~~~~~~~~~~~
`` ``

Correct. Because we have a quote, we need to give it a lifecycle. But our `factory ()` function does not accept parameters, so omitting can not be used here. What life cycle can we use? `` Static`:

`` `Rust
Fn factory () -> & 'static (Fn (i32) -> i32) {
    Let num = 5;

    X | x + num
}

Let f = factory ();

Let answer = f (1);
Assert_eq! (6, answer);
`` ``

But this will be another mistake:

`` `Text
Error: mismatched types:
 Expected `& 'static core :: ops :: Fn (i32) -> i32`,
    Found `[closure @ <anon>: 7: 9: 7:20]`
(Expected & -ptr,
    Found closure) [E0308]
         X | x + num
         ^ ~~~~~~~~~~

`` ``

This error tells us that we did not return a `` static Fn (i32) -> i32`, but instead returned a `[closure <anon>: 7: 9: 7:20]`. Wait, what?

Because each closure generates its own environment `struct` and implements` Fn` and some other things, these types are anonymous. They only exist in this closure. So Rust shows them as `closure <anon>` instead of some auto-generated names.

This error also indicates that the return value type expect is a reference, but we try not to return. Further, we can not directly give an object `` static` statement cycle. So we change a method and `box` box` Fn` to return a trait object. This * almost * can run successfully:

`` `Rust
Fn factory () -> Box <Fn (i32) -> i32> {
    Let num = 5;

    Box :: new (| x | x + num)
}
# Fn main () {
Let f = factory ();

Let answer = f (1);
Assert_eq! (6, answer);
#}
`` ``

This is the last question:

`` `Text
Error: closure may outlive the current function, but it borrows `num`,
Which is owned by the current function [E0373]
Box :: new (| x | x + num)
         ^ ~~~~~~~~~~
`` ``

Well, as we discussed above, the closures borrow their environment. And in this case. Our environment is based on a stack of `5`,` num` variables bound. So this borrow has this stack frame life cycle. So if we return this closure, the function call will end and the stack will disappear, and our closure will point to the freed memory environment! And then the last one to modify, we can let it run:

`` `Rust
Fn factory () -> Box <Fn (i32) -> i32> {
    Let num = 5;

    Box :: new (move | x | x + num)
}
# Fn main () {
Let f = factory ();

Let answer = f (1);
Assert_eq! (6, answer);
#}
`` ``

By adding the `move` keyword to the internal closures, we force the closures to use the move to capture the environment variables. Because the num type here is i32, in fact, the implementation of the move here is copy, so that the closure will no longer have a pointer to the environment, but the complete possession of the variables were captured. And allow it to leave our stack frame.

> ### This part is quoted from [The Rust Programming Language Chinese] (https://github.com/KaiserY/rust-book-chinese/blob/master/content/Closures%20%E9%97%AD%E5 % 8C% 85.md)