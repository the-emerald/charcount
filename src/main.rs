use std::env;
use std::fs::File;
use std::io::{Read};
use crate::charcount::CharCount;

mod charcount;

fn read_to_buffer(path: &String, mut buffer: &mut String) -> String{
    File::open(&path)
        .expect("Could not open file")
        .read_to_string(&mut buffer)
        .expect("Could not read to buffer").to_string()
}

fn main() {
    let encode;

    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        encode = false;
    }
    else if args.len() == 4 {
        encode = true;
    }
    else {
        panic!("Usage: ./encoder <input> [output_ciphertext] [output_key]")
    }

    // Read input file
    let mut buffer_input = String::new();
    read_to_buffer(&args[1], &mut buffer_input);

    if !encode {
        let mut count = CharCount::new();
        count.populate(&buffer_input);
        count.pretty_print();
    }
}