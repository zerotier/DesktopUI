extern crate crc64;
use crc64::crc64;
use std::env;
use std::path::Path;
use std::io::{Read,BufReader};
use std::fs::File;

pub fn main() {
    let mut args = env::args();
    let (len,_) = args.size_hint();
    let prog = args.next().unwrap();

    if len == 1 {
        println!("Usage: {} [list of files]", prog);
        return
    }

    for f in args {
        let mut crc : u64 = 0;
        let file = File::open(&Path::new(&f)).unwrap();
        let mut reader = BufReader::new(file);

        let mut error = false;
        loop {
            let mut buf = [0; 100];
            match reader.read(&mut buf) {
                Err(e) => {
                    error = true;
                    print!("error reading '{}': {}", f, e);
                    break;
                },
                Ok(0) => break,
                Ok(nread) => crc = crc64::crc64(crc, &buf[..nread])
            }
        }

        if error == false {
            println!("{:x}  {}", crc, f);
        }
    }
}
