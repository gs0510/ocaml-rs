# ocaml-rs - OCaml FFI for Rust

NOTE: `ocaml-rs` is forked from [raml v0.1](https://crates.io/crates/raml) with the goal of creating a higher-level interface.

```rust
#[macro_use]
use ocaml;
use ocaml::ToValue;

caml!(ml_beef, |parameter|, <local>, {
    let i = parameter.val_int();
    let res = 0xbeef * i;
    println!("about to return  0x{:x} to OCaml runtime", res);
    local = res.to_value();
} -> local);
```

The macro takes care of _automatically_ declaring `CAMLparam` et. al, as well as `CAMLlocal` and `CAMLreturn`

For a working example see `./examples/rust` and `./examples/ocaml`

### Documentation

https://docs.rs/ocaml/
