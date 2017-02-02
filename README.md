# acid-state
rust transactional state library

```rust
#[macro_use] extern crate acid-state;

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct A {
  i: u64,
}

acid_state! {
  pub a: A = A { i: 0 };
}

fn main() {
  println!("a initialized or loaded from disk is {}", *a);
  acid! {
    a.i += 1
  }
  println!("a is now {}", *a);
}
```
