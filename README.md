# rust-belt

### `dump!` macro

```rust

#[macro_use]
extern crate belt;

fn main() {
    let s = "hi";
    dump!(s);
}
```

will print `s => "hi"`