use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, MutexGuard};

use rustc_serialize::{Encodable, Decodable};

use bincode::SizeLimit;
use bincode::rustc_serialize::{encode, decode, DecodingResult};

fn to_binary<T: Encodable>(s: &T) -> Vec<u8> {
    encode(s, SizeLimit::Infinite).unwrap()
}

fn from_binary<T: Decodable>(encoded: Vec<u8>) -> DecodingResult<T> {
    decode(&encoded[..])
}

#[derive(Debug, Clone)]
pub struct Persistent<T: Encodable + Decodable> {
    pub inner: Arc<Mutex<T>>,
    pub name: String,
}

impl<T: Encodable + Decodable> Persistent<T> {
    pub fn handle<'a>(&'a self) -> Txn<'a, T> {
        let mut inner = self.inner.lock().unwrap();
        if let Some(read) = self.read() {
            *inner = read;
        }
        Txn {
            inner: inner,
            name: self.name.clone(),
        }
    }

    fn read(&self) -> Option<T> {
        if let Ok(mut f) = fs::File::open(self.path()) {
            let mut s = vec![];
            f.read_to_end(&mut s).unwrap();
            from_binary(s).ok().or_else(|| None)
        } else {
            None
        }
    }

    fn path(&self) -> PathBuf {
        self.name.clone().into()
    }

    fn clear(&self) -> io::Result<()> {
        fs::remove_file(self.path())
    }
}


impl<'a, T: 'a + Encodable + Decodable> Txn<'a, T> {
    fn path(&self) -> PathBuf {
        self.name.clone().into()
    }

    fn write(&self) -> io::Result<()> {
        let bytes = to_binary(&*self.inner);
        from_binary::<T>(bytes.clone()).unwrap();
        let mut f = fs::File::create(self.path()).unwrap();
        let res = f.write_all(&*bytes);
        f.sync_all();
        res
    }
}

pub struct Txn<'a, T: 'a + Encodable + Decodable> {
    pub inner: MutexGuard<'a, T>,
    pub name: String,
}

impl<'a, T: Encodable + Decodable + fmt::Debug> fmt::Debug for Txn<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Txn {{ inner: {:?} }}", *self.inner)
    }
}

impl<'a, T: Encodable + Decodable> Drop for Txn<'a, T> {
    fn drop(&mut self) {
        self.write();
    }
}

impl<'a, T: 'a + Encodable + Decodable> Deref for Txn<'a, T> {
    type Target = MutexGuard<'a, T>;

    fn deref(&self) -> &MutexGuard<'a, T> {
        &self.inner
    }
}

impl<'a, T: 'a + Encodable + Decodable> DerefMut for Txn<'a, T> {
    fn deref_mut(&mut self) -> &mut MutexGuard<'a, T> {
        &mut self.inner
    }
}
