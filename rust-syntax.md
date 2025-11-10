# Rust Syntax Reference

Learning journal for Rust syntax encountered while working through Zero2Prod.
Organized chronologically as concepts appear.

---

| Rust | Explanation | Scala Equivalent |
|------|-------------|------------------|
| `//!` | Inner doc comment - documents the containing module/crate itself | `/** package docs */` in package.scala |
| `///` | Outer doc comment - documents the item that follows | `/** method docs */` before method |
| `use` | Import items into scope | `import` |
| `::` | Path separator for modules/types/static functions | `.` for packages, objects |
| `.` | Method call on instance | `.` (same) |
| `&str` | String slice - immutable reference to string data | `String` (immutable by default) |
| `const` | Compile-time constant, always inlined, requires type annotation | `val` but stricter (must be compile-time) |
| `let` | Runtime variable binding (immutable by default), type inferred | `val` (evaluated at runtime) |
| `let mut` | Mutable variable binding | `var` |
| `!` (in `println!`) | Indicates a macro, not a function | N/A - no macro syntax in Scala |
| `format!` | Runtime string formatting macro (returns `String`) | `s"text $var"` string interpolation |
| `formatcp!` | Compile-time string formatting (for `const`) | N/A - const values handled differently |
| `#[attribute]` | Attribute macro - applies metadata/transformations to following item | Annotations like `@main`, `@tailrec` |
| `#[tokio::main]` | Sets up async runtime for main function | `extends IOApp` in cats-effect |
| `#[tokio::test]` | Sets up async runtime for test function (includes `#[test]`) | N/A - tests use `IO.unsafeRunSync()` |
| `async fn` | Function that returns a Future | `def foo: IO[A]` or `def foo: F[A]` |
| `.await` | Execute a Future (like forcing evaluation) | `.unsafeRunSync()` in cats-effect |
| `Result<T, E>` | Either success (Ok) or error (Err), left-biased | `Either[E, T]` (right-biased by convention) |
| `Ok(value)` | Success variant of Result | `Right(value)` |
| `Err(error)` | Error variant of Result | `Left(error)` |
| `?` operator | Early return on error; unwraps Ok, returns Err | Early exit in for-comprehension or `.flatMap` chain |
| `impl Trait` | Returns some type that implements Trait (opaque return) | Existential types or abstract type members |
| `trait` | Defines behavior contract (like interface) | `trait` (similar!) or typeclass |
| `->` | Function return type annotation | `:` before return type |
| `()` | Unit type (like void) | `Unit` |
| `{...}` | Block expression - returns value of last expression | `{...}` (same!) |
| `;` | Statement terminator - discards expression value | `;` (but Scala allows omission) |
| `\|\|` (double pipes) | Closure with zero arguments | `() => ...` |
| `\|a, b\|` | Closure with arguments | `(a, b) => ...` |
| `pub` | Makes item public (visible outside module) | N/A - everything public by default unless `private` |
| `.unwrap_or(default)` | Extract value from Option/Result or use default | `.getOrElse(default)` |
| `assert!` | Macro - panics if condition is false | `assert(condition)` |
| `assert_eq!` | Macro - panics if values not equal | `assertEquals` in test frameworks |
| `.expect("msg")` | Unwrap or panic with custom message | `.getOrElse(throw new Exception("msg"))` |
| `Some(value)` | Present value in Option | `Some(value)` (same!) |
| `None` | Absent value in Option | `None` (same!) |
| `tokio::spawn(future)` | Spawns future as background task on tokio runtime | `io.start` in cats-effect or `zio.fork` in ZIO |
| `String` | Owned, heap-allocated, growable string type | `StringBuilder` or mutable string buffer |
| `&str` vs `String` | `&str` is borrowed reference (doesn't own), `String` owns data | In Scala, `String` is always a reference (GC-managed) |
| `&expr` | Borrow operator - creates reference to a value | N/A - everything already a reference in Scala |
| `'static` | Lifetime annotation - data lives for entire program duration | N/A - GC handles lifetime |
| `'a` (lifetime) | Lifetime parameter - ties borrowed data's lifetime to scope | N/A - GC manages this |
| Deref coercion | `String` automatically converts to `&str` when `&str` expected | Implicit conversions |
| `&format!(...)` | Borrow a temporary String as &str. `format!` returns owned String, `&` borrows it as &str for the expression scope. Common when APIs expect &str but you need runtime formatting | `s"text $var"` produces String directly; in Scala everything is already a reference |
| `vec![a, b, c]` | Macro to create Vec<T> (growable array) with initial elements | `List(a, b, c)` or `Vector(a, b, c)` |
| `(a, b)` | Tuple literal - fixed-size heterogeneous collection | `(a, b)` (same!) |
| `for (a, b) in vec` | Destructuring pattern in for loop (iterates over collection) | `for ((a, b) <- list)` |
| `.as_u16()` | Convert type to u16 primitive (explicit conversion) | `.toInt`, `.toLong` etc |
| `assert_eq!(a, b, "msg")` | Assert equality with custom failure message (3-arg form) | `assertEquals(a, b, "msg")` |

---

## Notes:
- **Futures are lazy in Rust** - they do nothing until `.await`ed (like `IO` in cats-effect)
- **Result is left-biased** - `Ok` is the success path (opposite convention from Scala's `Either`)
- **Immutable by default** - `let` creates immutable bindings (like Scala's `val`)
- **Macros end with `!`** - compile-time code generation
- **Const requires compile-time evaluation** - can't use runtime functions like `format!`

## Core Concepts:
- **Ownership** - Every value has one owner; when owner goes out of scope, value is dropped
- **Borrowing** - Can create references (`&T`) to use values without taking ownership
- **Lifetimes** - Compiler tracks how long references are valid to prevent dangling pointers
- **String vs &str** - Use `&str` for function params (flexible), `String` for owned/returned data
