# Code Styling

Almost all code should follow default `rustfmt` styling convenstions, with one
exception. There should be no trailing commas on multiline lists. This is
already configured in the `rustfmt.toml` file, however this styling feature
is only in the nightly version as of 2021-09-25. To format using this style
run the comand as `cargo +nightly fmt`. You must already have the nightly Rust
version install using rustup.

For example something like this:
```rust
struct Foo {
  foo: u32,
  bar: u32, // bad trailing comma
}
```
should be replaced with this
```rust
struct Foo {
  foo: u32,
  bar: u32 // No trailing comma here
}
```
