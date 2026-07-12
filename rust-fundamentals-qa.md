# Rust Fundamentals — Q&A

Notes from working through the rustlings `03_if/if1.rs` exercise.

---

## 1. Why are Rust variables immutable by default?

```rust
let x = 5;
x = 6; // ❌ compile error: cannot assign twice to immutable variable `x`
```

It's a **safety-by-default** design choice. Immutability means:

- **The compiler catches accidental mutation** — if you didn't *intend* to change a value, an accidental write becomes a compile error instead of a silent bug.
- **Fearless concurrency** — immutable data can be shared across threads without data races. This is the cornerstone of Rust's concurrency guarantees.
- **Easier reasoning** — you can read `let x = compute();` and know `x` never changes for the rest of its scope.

> **Insight:** "Immutable by default" ≠ "constant." A `const` is compile-time and truly fixed; an immutable `let` is a runtime value that simply *can't be reassigned* after binding.

---

## 2. What does `mut` allow?

`mut` opts a binding *into* mutability — it lets you reassign or mutate the value in place:

```rust
let mut count = 0;
count += 1;      // ✅ allowed because of `mut`
count = 100;     // ✅ also allowed
```

Without `mut`, both lines fail to compile. It's an explicit, visible marker: anyone reading the code sees `mut` and knows "this value is expected to change."

---

## 3. Why is `f64` appropriate for temperature?

Temperature is a **continuous, fractional** quantity — 36.6°C, -40.5°, 98.6°F. You need a floating-point type, not an integer.

`f64` (64-bit double precision) over `f32` because:

- **It's Rust's default float** — a bare `let t = 36.6;` infers `f64`.
- **Precision** — ~15–17 significant digits vs. `f32`'s ~6–7, so conversions and arithmetic (like °C↔°F) don't accumulate visible rounding error.
- On modern 64-bit CPUs there's rarely a meaningful performance penalty, so the extra precision is nearly free.

You'd only reach for `f32` when memory/bandwidth matters a lot (huge arrays, GPU, embedded).

---

## 4. Statement vs. expression

This is *the* key distinction in Rust, and it's what makes `if1.rs` solvable.

|                     | **Expression**                                              | **Statement**                        |
| ------------------- | ---------------------------------------------------------- | ------------------------------------ |
| Produces a value?   | **Yes**                                                    | No                                   |
| Examples            | `5`, `a + b`, `if a > b { a } else { b }`, a block `{ ... }` | `let x = 5;`, a `fn` definition       |
| Ends with `;`?      | No (adding `;` turns it into a statement)                   | Yes                                  |

```rust
let y = {
    let inner = 3;
    inner + 1     // ← expression (no semicolon): the block evaluates to 4
};                // ← the `let` is a statement
```

The semicolon is the switch: `inner + 1` is an expression worth `4`; `inner + 1;` is a statement worth *nothing* (`()`, the unit type).

> **Insight:** In Rust `if` is an **expression**, not just control flow. `let max = if a > b { a } else { b };` is idiomatic — impossible in C/Java where `if` is a statement.

---

## 5. Why can a Rust function return its final expression without `return`?

Because a function body is a **block**, and a block evaluates to its final expression (see #4). The function's return value *is* that block's value:

```rust
fn bigger(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }   // final expression, no `;`, no `return`
}
```

- No trailing semicolon → this `if` expression's value becomes the function's return value.
- `return a;` is the *early* / *explicit* form — needed to bail out mid-function, but noise at the end.
- **Watch the semicolon trap:** writing `if a > b { a } else { b };` makes it a statement worth `()`, and you'd get a type-mismatch error (`expected i32, found ()`).
