JSON macro for Rust
====

## Usage

This will be uploaded to crates.io when it is more stable.
For now, add the following to your `Cargo.toml`

```toml
[dependencies.json_macro]
git = "https://github.com/fuchsnj/json_macro"
```

and this to your crate root:

```rust
#[macro_use]
extern crate json_macro;
```

```rust
let value = 4;
let json = json!({
	"a" => "1",
	"b" => (value)
});
```
