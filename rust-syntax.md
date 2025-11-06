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
| `const` | Compile-time constant (must be computable at compile time) | `val` at top level |
| `let` | Variable binding (immutable by default) | `val` |
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

---

## Notes:
- **Futures are lazy in Rust** - they do nothing until `.await`ed (like `IO` in cats-effect)
- **Result is left-biased** - `Ok` is the success path (opposite convention from Scala's `Either`)
- **Immutable by default** - `let` creates immutable bindings (like Scala's `val`)
- **Macros end with `!`** - compile-time code generation
- **Const requires compile-time evaluation** - can't use runtime functions like `format!`
