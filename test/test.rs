#[macro_use]
extern crate acid_state;
#[macro_use]
extern crate lazy_static;
extern crate rustc_serialize;

use std::collections::HashMap;

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct Cs {
    v: u64,
}

acid_state! {
    A: HashMap<String, u64> = HashMap::new();
    B: u64 = 0;
    C: Cs = Cs {
        v: 0
    };
}

#[test]
fn it_works() {
    let key = "yo".to_owned();
    acid! {
        (A => a, B => b, C => c) => {
            // A, B, C have been pulled off disk
            let current = a.entry(key).or_insert(0);
            **b += 1;
            *current += 1;
            c.v += 1;
            println!("a is now {:?}", current);
            println!("b is now {:?}", **b);
            println!("c is now {:?}", c.v);
            // new values of A, B, C are now synced on disk
        }
    };
}
