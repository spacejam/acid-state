//! `acid-state` adds durable transaction support to any serializable structure.
#![crate_type = "lib"]
// #![deny(missing_docs)]

//! Transactional state.
//!
//! # Examples
//!
//! ```ignore
//! #[macro_use]
//! extern crate acid_state;
//! #[macro_use]
//! extern crate lazy_static;
//! extern crate rustc_serialize;
//!
//! use std::collections::HashMap;
//!
//! #[derive(Debug, RustcDecodable, RustcEncodable)]
//! struct Cs {
//!    v: u64,
//! }
//!
//! acid_state! {
//!    A: HashMap<String, u64> = HashMap::new();
//!    B: u64 = 0;
//!    C: Cs = Cs {
//!        v: 0
//!    };
//! }
//!
//!  let key = "yo".to_owned();
//!  acid! {
//!      (A => a, B => b, C => c) => {
//!          // A, B, C have been pulled off disk
//!          let mut current = a.entry(key).or_insert(0);
//!          **b += 1;
//!          *current += 1;
//!          c.v += 1;
//!          println!("a is now {:?}", current);
//!          println!("b is now {:?}", **b);
//!          println!("c is now {:?}", c.v);
//!          // new values of A, B, C are now synced on disk
//!      }
//!  };
//! ```

pub use std::sync::{Arc as __ARC, Mutex as __MUTEX};

#[macro_use]
extern crate lazy_static;
extern crate rustc_serialize;
extern crate bincode;

mod wrapper_structs;
pub use wrapper_structs::{Persistent, Txn};

#[macro_export]
macro_rules! acid_state {
    ($N:ident : $T:ty = $e:expr; $($rest:tt)*) => {
        lazy_static!(
            static ref $N : $crate::Persistent<$T> =
                $crate::Persistent {
                    inner: $crate::__ARC::new($crate::__MUTEX::new($e)),
                    name: format!("{}.acidstate", stringify!($N)).into(),
                };
        );
        acid_state!($($rest)*);
    };
    () => ()
}

// begin txn locking [idents], body with idents handle()'d, persist, unlock [idents]
#[macro_export]
macro_rules! acid {
    (($($src:ident => $dst:ident),*) => $b:block) => {
        {
            $(let mut $dst = $src.handle();)*
            $b
        }
    };
}
