#![feature(slicing_syntax)]

extern crate crc64;
extern crate time;
use crc64::crc64;
use std::os;
use std::io::File;
use time::get_time;

fn ustime() -> u64 {
    let tv = time::get_time();

    let mut res = (tv.sec as u64) * 1000000000;
    res += tv.nsec as u64;

    return res
}

fn main() {
    let args = os::args();
    if args.len() == 1 {
        println!("Usage: {} [list of files]", args[0]);
        panic!();
    }

    let f = args[1].clone();
    let mut crc : u64 = 0;
    let content = File::open(&Path::new(f.to_string())).read_to_end().unwrap();

    /*
    let table = crc64::crc64_init();

    println!("[");

    for t in table.iter() {
        println!("[");
        let mut i = 0;
        for x in t.iter() {
            print!("0x{:0>16x}, ", x);
            i = i + 1;
            if i == 2 {
                println!("");
                i = 0;
            }
        }
        println!("],");
    }
    println!("]");

    //println!("table\n{:?}", table);
    return;
    */

    let sz = content.len();
    let size_mb = sz as f64 / 1024.0 / 1024.0;

    println!("CRC for {} MB file", size_mb);

    let start = ustime();
    crc = crc64::crc64(crc, content.as_slice());
    let end = ustime();

    let total_time_seconds = (end-start) as f64 / 1e9;
    let speed = size_mb / total_time_seconds as f64;

    println!("{:x}  {}", crc, f);
    println!("{} seconds at {} MB/s", total_time_seconds, speed);
}
