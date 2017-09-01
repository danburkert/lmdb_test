extern crate lmdb;
extern crate tempdir;
extern crate byteorder;

use std::path::Path;

use byteorder::{ByteOrder, LittleEndian};
use lmdb::{Database, Environment, Transaction};
use tempdir::TempDir;

const HEIGHT_KEY: [u8; 1] = [0];

fn main() {
    let dir = TempDir::new("test").unwrap();
    let db = LMDB::new(dir.path(), 1_000_000_000);

    for height in 0..100000 {
        eprintln!("height: {}", height);
        db.save_height(height);
    }
}

#[derive(Debug)]
pub struct LMDB {
    pub env: Environment,
    pub index: Database,
}

impl LMDB {
    // add code here
    pub fn new(path: &Path, size: usize) -> Self {
        let mut builder = Environment::new();
        builder.set_max_dbs(2);
        builder.set_map_size(size);
        let env = builder.open(path).expect("open lmdb env");
        let index = env.create_db(None, lmdb::DUP_SORT).expect("open index db");

        LMDB {
            env: env,
            index: index,
        }
    }

    pub fn save_height(&self, height: u64) {
        let mut value = [0u8; 8];
        LittleEndian::write_u64(&mut value, height);
        let mut tx = self.env.begin_rw_txn().expect("begin_rw_txn");
        tx.put(self.index,
               &HEIGHT_KEY,
               &value,
               lmdb::WriteFlags::empty()).expect("tx.put");
        tx.commit().expect("ts.commit")
    }
}
