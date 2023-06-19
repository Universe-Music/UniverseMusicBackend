mod core;
mod utils;

use crate::core::db::DBPool;
use crate::core::scanner::file::Files;
use crate::core::scanner::prober;
use crate::utils::log::init_log;
use std::fs::File;
#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    init_log();
    let f = Files::new("/mnt/d/音乐");
    let mut files = match f {
        Err(e) => {
            println!("{:?}", e);
            return;
        }
        Ok(t) => t,
    };
    loop {
        let err = match files.next() {
            None => break,
            Some(t) => prober::probe(File::open(&t).unwrap(), &t),
        };
        match err {
            Err(e) => println!("{:?}", e),
            Ok(t) => println!("{:?}", t),
        }
    }
    println!("{:?}", files.get_errors());
}
