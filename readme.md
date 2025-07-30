Below is the same **Learn Rust in 21 Days** outline, but every code sample has been replaced by a short placeholder so you can focus on concepts first. Whenever you see **\[code hidden]**, that’s where the original example lived—feel free to write your own version there or ask me for a hint later.

---

## Day 1 – Introduction to Rust & “Hello World”

**Topics**

* Installing Rust and using Cargo
* Anatomy of a minimal Rust program
* Compiling vs. running
* Project layout and comments

**Key Points**

* Every Rust program starts in `fn main()`
* `println!` is a macro; the `!` matters
* Use `cargo run` to compile *and* execute

**Quiz & Challenge**

* Commands to create/run a Cargo project
* Modify “Hello World” to print a second line
* Mini‑task: print your name and today’s date

**\[code hidden]**

---

## Day 2 – Variables & Mutability

**Topics**

* Declaring with `let`, immutability by default
* Adding `mut` when a value must change
* Constants with `const`
* Shadowing vs. mutability

**Key Points**

* Immutability prevents accidental changes
* The compiler flags illegal reassignments early

**Quiz & Challenge**

* Explain why Rust defaults to immutability
* Experiment with a compiler error by trying to mutate an immutable variable, then fix it

**\[code hidden]**

---

## Day 3 – Data Types

**Topics**

* Scalar types: integers, floats, booleans, chars
* Compound types: tuples, arrays, slices
* Type inference vs. explicit annotations

**Key Points**

* Arrays have fixed length; vectors (later) are resizable
* Default types: `i32` for integers, `f64` for floats

**Quiz & Challenge**

* Create a 3‑D point tuple and a weekday array
* Observe a runtime panic by indexing out‑of‑bounds

**\[code hidden]**

---

## Day 4 – Control Flow (I): `if` / `else`

**Topics**

* Boolean conditions (no truthy/falsy)
* `if … else if … else` chains
* `if` as an expression replacing the ternary operator
* Comparison & logical operators

**Key Points**

* Both branches of an `if` expression must return the same type
* No automatic integer‑to‑bool conversion

**Quiz & Challenge**

* Classify ages into “Child”, “Teen”, “Adult” using `if` expressions

**\[code hidden]**

---

## Day 5 – Control Flow (II): Loops

**Topics**

* Infinite `loop`, exiting with `break`
* `while` loops
* `for` loops over ranges & collections
* Loop labels, `continue`, returning a value from `break`

**Key Points**

* Prefer `for` for safe, index‑free iteration
* Loop labels let you break out of specific nested loops

**Quiz & Challenge**

* Print the first power of 2 greater than 1000 using a loop
* Generate the first 10 Fibonacci numbers with a `for` loop

**\[code hidden]**

---

## Day 6 – Functions

**Topics**

* Defining functions with `fn`
* Parameters and explicit types
* Return types with `->`
* Statements vs. expressions; semicolon rules
* Early `return`

**Key Points**

* The last expression (without `;`) is the return value
* Functions promote reuse and clearer code structure

**Quiz & Challenge**

* Write `fahrenheit_to_celsius` and `is_even` without seeing the sample code

**\[code hidden]**

---

## Day 7 – Ownership

**Topics**

* The three ownership rules
* Stack vs. heap overview
* Move semantics, `Clone`, `Copy`
* Ownership transfer in function calls

**Key Points**

* Each value has one owner; moving invalidates the original variable
* Types like integers are `Copy`; `String` is not

**Quiz & Challenge**

* Observe a move error, then fix it with `.clone()`
* Predict which common types implement `Copy`

**\[code hidden]**

---

## Day 8 – References & Borrowing

**Topics**

* Immutable references `&T`
* Mutable references `&mut T`
* Borrowing rules (one writer / many readers)
* Dangling references and lifetimes (intro only)

**Key Points**

* Borrow to read without taking ownership
* Cannot mix mutable and immutable borrows simultaneously

**Quiz & Challenge**

* Write `add_excitement(text: &mut String)`
* Experiment with borrowing rules by interleaving `&` and `&mut`

**\[code hidden]**

---

## Day 9 – Structs & Methods

**Topics**

* Defining `struct` types and creating instances
* Field access and update syntax
* Implementing methods with `impl` (`&self`, `&mut self`)
* Associated functions (`::new` pattern)

**Key Points**

* Structs bundle data; methods bundle behavior
* Ownership of fields follows normal rules inside a struct

**Quiz & Challenge**

* Implement `Rectangle` with `area`, `can_hold`, and `square` constructor

**\[code hidden]**

---

## Day 10 – Enums & Basic Pattern Matching

**Topics**

* Defining enums with and without data
* The ubiquitous `Option<T>` enum
* Exhaustive `match` expressions
* The `_` wildcard and `if let` sugar

**Key Points**

* Enums + `match` provide safe, explicit branching
* `Option<T>` replaces null by forcing you to handle `None`

**Quiz & Challenge**

* Implement `Coin` value matcher and a `Shape` perimeter calculator

**\[code hidden]**

---

### Days 11–21 (Coming Up)

Over the remaining 11 days you’ll cover:

* Day 11 – Advanced pattern matching & guards
* Day 12 – Collections (`Vec`, `String`, `HashMap`)
* Day 13 – Error handling (`Result`, `panic!`, `unwrap`, `expect`)
* Day 14 – Modules & the package ecosystem (Cargo crates)
* Day 15 – Generic functions & structs
* Day 16 – Traits & trait bounds
* Day 17 – Lifetimes in depth
* Day 18 – Closures & iterators
* Day 19 – Testing (unit, integration)
* Day 20 – Basic concurrency (threads, channels)
* Day 21 – Final project: a CLI **contact book** tying everything together

Each of those days will follow the same structure: concise explanations, **\[code hidden]** placeholders for you to implement, short quizzes, and a challenge that pushes you a bit beyond the day’s examples.

---

**How to Proceed**

1. Take one day at a time—read the explanations, then write your own code where placeholders appear.
2. Lean on the compiler: if it complains, treat the message as a hint.
3. After practicing, circle back with questions or request the hidden example for any snippet you’re stuck on.

Happy Rust‑ing!
