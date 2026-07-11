# Rust Learning Plan: Weeks 1-2

## Goal

Spend 5-7 hours per week learning Rust through small exercises and one useful
command-line program. At the end of week 2, you should be able to explain basic
ownership and borrowing, handle recoverable errors, and build and test a small
file-analysis tool.

## Working rhythm

Use five sessions per week:

- Four 45-60 minute learning and coding sessions
- One 60-90 minute review and project session

For every session:

1. Read or watch material for no more than 20 minutes.
2. Type the examples yourself.
3. Change the example and predict what the compiler will do.
4. Record one lesson or compiler error in `LEARNING.md`.

Do not automatically fix ownership problems with `.clone()`. First identify
which value owns the data, which code only needs to borrow it, and how long each
reference must remain valid.

## Week 1: Core syntax, ownership, and borrowing

### Day 1: Cargo and basic Rust syntax

Learn:

- `fn`, `let`, `mut`, constants, and scalar types
- Expressions versus statements
- `cargo run`, `cargo check`, and `cargo build`

Practice:

- Replace the starter program with temperature conversions between Celsius and
  Fahrenheit.
- Add functions for both conversion directions.
- Try immutable and mutable variables and read the compiler messages.

Done when:

- The program produces correct conversions for at least three inputs.
- `cargo check` passes.

### Day 2: Control flow and functions

Learn:

- `if`, `loop`, `while`, and `for`
- Function parameters and return values
- Ranges and arrays

Practice:

- Write a FizzBuzz program for the numbers 1 through 100.
- Move the FizzBuzz decision into a function that returns a `String`.
- Write tests for 3, 5, 15, and an ordinary number.

Done when:

- The tests pass with `cargo test`.
- You can explain why the final expression of a function does not need
  `return` or a semicolon.

### Day 3: Ownership and moves

Learn:

- Stack and heap data
- Ownership and scope
- Move semantics and the `Copy` trait
- The difference between assignment, moving, and cloning

Practice:

- Pass a `String` into a function and observe the move.
- Return the `String` to transfer ownership back.
- Compare this behavior with an `i32`.
- Use `.clone()` once, then write down why it creates additional data.

Done when:

- You can predict whether a value remains usable after a function call.
- You can explain why `String` moves while `i32` is normally copied.

### Day 4: References, borrowing, and slices

Learn:

- Shared references: `&T`
- Mutable references: `&mut T`
- Borrowing rules
- String slices: `&str`

Practice:

- Write `word_count(text: &str) -> usize`.
- Write `first_word(text: &str) -> &str` without allocating a new `String`.
- Write `normalize(text: &mut String)` to trim and lowercase text.
- Intentionally create two overlapping mutable borrows and study the error.

Done when:

- All three functions work and have tests.
- You can state the rule: many shared references or one mutable reference at a
  time.

### Day 5: Week 1 review

Without looking at earlier code, write a small program that:

1. Accepts a hard-coded sentence.
2. Counts its words.
3. Finds its first word.
4. Prints the normalized sentence.

Review questions:

- Who owns each `String`?
- Which functions borrow instead of taking ownership?
- When would returning a `String` be preferable to returning `&str`?
- What is the difference between `String` and `&str`?

Week 1 verification:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

## Week 2: Data modeling, collections, errors, and file I/O

### Day 6: Structs and methods

Learn:

- Struct definitions and instances
- Associated functions and methods
- Borrowing `self` with `&self` and `&mut self`

Practice:

- Create a `TextStats` struct containing line, word, and character counts.
- Implement a constructor that analyzes `&str`.
- Add a method that formats a short report.

Done when:

- Creating `TextStats` does not require the struct to own the input text.
- Tests verify the counts for empty and multi-line input.

### Day 7: Enums, pattern matching, and `Option`

Learn:

- Enums and enum data
- `match` and `if let`
- `Option<T>`

Practice:

- Add a function that returns the most frequent word as `Option<(&str, usize)>`
  or, if borrowing becomes distracting, `Option<(String, usize)>`.
- Handle empty input without panicking.
- Use `match` to print either the result or `No words found`.

Done when:

- Empty input returns `None`.
- Non-empty input returns `Some` with the expected word and count.

### Day 8: Collections and iterators

Learn:

- `Vec<T>` and `HashMap<K, V>`
- Iterator adapters such as `map`, `filter`, and `count`
- Consuming versus borrowing iterators

Practice:

- Use a `HashMap<String, usize>` to count word frequencies.
- Normalize case and ignore basic punctuation.
- Produce the five most common words.

Done when:

- The original text remains available after analysis.
- Tests cover repeated words, mixed case, and punctuation.

### Day 9: `Result`, filesystem access, and command-line arguments

Learn:

- `Result<T, E>` and the `?` operator
- `std::fs::read_to_string`
- `std::env::args`
- Writing useful error messages

Practice:

- Accept a file path as the first command-line argument.
- Read the file and print `TextStats`.
- Return an error instead of calling `unwrap()` for missing arguments or files.

Example:

```bash
cargo run -- README.md
```

Done when:

- A valid file prints a report.
- A missing argument and nonexistent file both fail with understandable errors.
- The program does not panic for expected user errors.

### Day 10: Finish the `rstats` milestone

The first useful tool should support:

```text
rust-tools <FILE>
```

It should report:

- Number of lines
- Number of words
- Number of characters
- Most frequent word, when one exists

Final tasks:

1. Separate analysis logic from argument parsing and printing.
2. Add unit tests for the analysis logic.
3. Add one integration-style test by running the program manually against a
   sample file.
4. Update the repository README with usage examples.
5. Create a release build.

Week 2 verification:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo build --release
./target/release/rust-tools README.md
```

## Definition of success

At the end of week 2, you should be able to:

- Describe ownership, moves, borrowing, and shared versus mutable references.
- Choose between `String` and `&str` for basic function interfaces.
- Model data with a struct and behavior with methods.
- Use `Option` when a value might be absent.
- Use `Result` and `?` for recoverable errors.
- Read a file, analyze it, and test the analysis independently from the CLI.
- Run formatting, linting, tests, and a release build without warnings.

If any of these remain unclear, repeat the associated exercise before moving
to traits, generics, lifetimes, concurrency, or async Rust.
