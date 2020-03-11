use std::env;
use std::fs::{File, read_to_string};
use std::io::{Write};
use crate::charcount::CharCount;
use ascii::{ToAsciiChar, AsciiString};
use std::collections::HashMap;
use rand::seq::SliceRandom;
use libc::isspace;
use std::str::FromStr;

mod charcount;


unsafe fn convert(c: char) -> Option<char> {
    let ascii_char = c.to_ascii_char().expect("Could not convert to ASCII");
    return if isspace(ascii_char as i32) != 0 { // Is a space so treat it as a space
        Some(' ')
    } else if ascii_char.is_alphabetic() {
        Some(c)
    } else {
        None
    }
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
        panic!("Usage: ./encoder <input> [output_ciphertext] [output_key]");
    }

    // Read input file
    let buffer_input = read_to_string(&args[1]).expect("Could not read file");

    if !encode { // Do a CharCount
        let mut count = CharCount::new();
        count.populate(&buffer_input);
        count.pretty_print();
    }
    else { // Perform encoding
        let mut original = (b'a'..=b'z') // Generate all ascii characters
            .map(|x| x as char)
            .collect::<Vec<_>>();
        original.push(' ');

        let mut shuffled = original.clone(); // Clone and shuffle
        shuffled.shuffle(&mut rand::thread_rng());

        println!("KEY: ");
        println!("{:?}", original);
        println!("{:?}", shuffled);

        let mut map: HashMap<char, char> = HashMap::new();
        for n in 0 as usize..27 as usize { // Generate a mapping of ascii to ascii
            map.insert(original[n], shuffled[n]);
        }

        let mut cipher_output = AsciiString::new();
        for c in buffer_input.chars() { // Generate ciphertext using mapping
            let use_char= match unsafe { convert(c) } {
                Some(t) => t.to_ascii_lowercase(),
                None => continue
            };
            cipher_output.push(map.get(&use_char).expect("Could not find in mapping!").to_ascii_char().unwrap());
        }

        let mut key_output = String::new();
        for c in buffer_input.chars() { // Generate key output using ctype rules
            let push_char= match unsafe { convert(c) } {
                Some(t) => t.to_ascii_lowercase(),
                None => continue
            };
            key_output.push(push_char);
        }
        let mut b = key_output.into_bytes();
        b.shuffle(&mut rand::thread_rng()); // Shuffle key
        key_output = String::from_utf8(b).unwrap();

        let key_output_ascii = AsciiString::from_str(&key_output).unwrap();

        // Write to files
        let mut cipher = File::create(&args[2]).unwrap();
        cipher.write_all(cipher_output.as_ref()).unwrap();
        println!("Written ciphertext to {}", &args[2]);

        let mut key = File::create(&args[3]).unwrap();
        key.write_all(key_output_ascii.as_ref()).unwrap();
        println!("Written key to {}", &args[3]);
    }
}