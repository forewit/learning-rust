# Getting started

https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

```
$ cargo new <name>  # Create a new cargo project
$ cargo build       tags: release, debug
$ cargo check       # verifies that the code compiles
$ cargo run
```

---

Variables are immutable by default. Use `mut` to make them mutable: `let mut name = String::new();`. Variable types can be infered or explicitly declared.

Constants must have a defined type: `const MAX_POINTS: u32 = 100_000;`

`match <expression> { <case> => <action> }` is similar to 'switch' in other languages.

| Rust     | Description        |
| -------- | ------------------ |
| `::`     | function of a type |
| `: f32`  | sets the type      |
| `&`      | reference          |
| `*`      | dereference        |
| `mut`    | mutable            |
| `1..10`  | range 1-9          |
| `1..=10` | range 1-10         |

"Shadowing" allows you to override a variable in a scope (declaring with the same name). For example:

```rust
let spaces = "   ";
let spaces = spaces.len();
// spaces is now "3" in this scope

let mut spaces = "   ";
let spaces = spaces.len();
// this will not compile
```

**Expressions vs statements**: Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value.

Function definitions are statements.

Expressions do NOT end with a semicolon!

```rust
let x = 5; // statement
fn main() { let x = 5; } // statement

{ let y = 3; y + 1 } // expression. returns 4
```

Return values from loops by adding the return after the break statement:

```rust
let mut counter = 0;

let result = loop {
    counter += 1;
    if counter == 10 {
        break counter*2;
    }
}; // result = 20
```

## [Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
Data in the **stack** must be a known size, but the **heap** can grow and shrink.

Values passed to functions are *pushed* onto the stack and *popped* off when the function completes.

Allocating to the heap returns a *pointer* and is expensive. **Ownership** is there to help manage memory on the heap:

> 1. Each value in Rust has a variable that’s called its owner.
> 2. There can only be one owner at a time.
> 3. When the owner goes out of scope, the value will be dropped.

For types that reference memory on the heap, "moving" a variable will invalidate the old owner:

```rust
// MOVE:
let s1 = String::from("hello");
let s2 = s1; 
// s1 is no longer valid. It is owned by s2.
// This is known as "move".
```
For types that store memeory on the stack, "copying" a variable will not invalidate the old owner:
```rust
// COPY:
let x = 5;
let y = x;
// x is still valid. because its memory can
// be quickly copied from the stack.
```

Passing variables to a function also transfers ownership.

## [References](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
References "borrow" a value from another variable without owning it.
You cannot modify a borrowed value!

> 1. At any given time, you can have either one mutable reference or any number of immutable references.
> 2. References must always be valid.

```rust
fn main() {
    let s = String::from("hello");

    // "&s" is a reference that borrows s.
    change(&s); 
}

fn change(some_string: &String) {
    some_string.push_str(", world");
    // this will not compile because you
    // cannot modify a borrowed value.
}
```

To get around this restriction you can use mutable references `&mut`. But you can only have one mutable reference at a time:
```rust
let mut s = String::from("hello");

let r1 = &mut s; // no problem
let r2 = &mut s; // BIG PROBLEM

println!("{}, {}", r1, r2);
```
Consider this example:
```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);
```
But this is OK:
```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{} and {}", r1, r2);
// r1 and r2 are no longer used after this point

let r3 = &mut s; // no problem
println!("{}", r3);
```

All this logic prevents dangling pointers! 🎉

## Moving on

Slices let you reference parts of a variable:

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];

s.clear();

// INVALID because 'hello' is a reference to
// part of a variable that is no longer valid:
println!("{}", hello); 
```