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

**Expressions vs statements:**
Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value.

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
