use std::collections::BTreeMap;
use ascii::{AsciiChar, ToAsciiChar};
use libc::isspace;

pub struct CharCount{
    count: BTreeMap<AsciiChar, usize>,
    count_spaces: usize
}
impl CharCount {
    pub fn new() -> Self {
        Self {
            count: Default::default(),
            count_spaces: 0
        }
    }
    
    pub fn populate(&mut self, buffer: &String) {
        for c in buffer.chars() {
            let ascii_char = c.to_ascii_char().expect("Could not convert to ASCII");
            unsafe {
                if isspace(ascii_char as i32) != 0 {
                    self.count_spaces += 1;
                }
                else if ascii_char.is_alphabetic() {
                    *self.count.entry(ascii_char.to_ascii_lowercase()).or_insert(0) += 1;
                }
            }
        }
    }

    pub fn pretty_print(&self) {
        println!("_, {}", self.count_spaces);
        for (ch, count) in &self.count {
            println!("{}, {}", ch, count);
        }
    }
}