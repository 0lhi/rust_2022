# Rust DLC
Mini-Library for Rust with quality-of-life stuff.

# How to embed

Add this to cargo.toml:

```toml
[dependencies]
rdlc = { git = "https://github.com/0lhi/rust_dlc.git", package = "rust_dlc"}
```

# Example for use

We have an ```input``` variable containing the String ```42.8 kg```. We want to get the number out of this and turn it into a floating point.

```rust
let number: f64 = input.trim().replace(|c: char| !rdlc::digital_scale(c) || c == ' ', "").trim().parse().unwrap();
```

Now we have:

```rust
number == 42.8_f64
```
