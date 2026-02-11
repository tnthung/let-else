# let-else

Quality-of-life macro for Rust `let-else` that lets you bind the else value to an identifier or a
pattern. It also supports a full `match` block for the else branch when you need per-variant logic.

## Features

- Bind the else value to a name or pattern.
- Support `|`-separated patterns in the else binding.
- Provide a full `match` for advanced else handling.
- Evaluate the input expression once.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
let-else = "0.2"
```

## Syntax

```rust
let_else!(PATTERN = EXPR else as REST { /* else block */ });
let_else!(PATTERN = EXPR else match { /* match arms */ });
```

`REST` can be a pattern (including `|`-separated patterns) or a single identifier.

## Examples

```rust
use let_else::let_else;

fn native(value: Result<i32, String>) {
  let Ok(x) = value else {
    // unable to access `Err(e)` here without extra `match` block
    return;
  };
}

fn with_macro(value: Result<i32, String>) {
  let_else!(Ok(_x) = value else as err {
    // bind everything else as `err`
    eprintln!("Error: {:?}", err);
    return;
  });
}

fn with_macro2(value: Result<i32, String>) {
  let_else!(Ok(_x) = value else as Err(e) {
    // bind only inner value of `Err` as `e`
    eprintln!("Error: {}", e);
    return;
  });
}

enum MoreVariant {
  A(i32),
  B(i32),
  C(i32),
}

fn with_macro3(value: MoreVariant) {
  use MoreVariant::*;

  let_else!(A(_x) = value else as B(y) | C(y) {
    // bind inner value from either `B` or `C` as `y`
    // this works because the `y` in B and C share identifier and type
    println!("Not A, but B or C with value: {}", y);
    return;
  });
}

fn with_match(value: MoreVariant) {
  use MoreVariant::*;

  // this provides more fine-grained control over each pattern
  let_else!(A(_x) = value else match {
    B(b) => {
      println!("Not A, but B with value: {}", b);
      return;
    },
    C(c) => {
      println!("Not A, but C with value: {}", c);
      return;
    },
  });
}
```

## License

MIT. See LICENSE.
