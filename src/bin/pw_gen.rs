use std::io::stdin;
use std::ops::{RangeInclusive};
use rand::Rng;

fn main() {
    let lowercase_chars = RangeInclusive::from(97 ..= 97 + 26 - 1)
        .map(|c: u8| c as char).collect::<Vec<char>>();
    let uppercase_chars = lowercase_chars.iter()
        .map(|c| c.to_ascii_uppercase())
        .collect::<Vec<char>>();
    let digits = RangeInclusive::from(0..=9).collect::<Vec<i32>>();
    let symbols = vec!['@', '=', '-'];

    println!("Enter password length:");
    let mut pw_len = String::new();
    stdin().read_line(&mut pw_len).expect("Error reading buffer");
    let pw_len = pw_len.trim().parse::<u32>().expect("Could not parse as u32");

    let mut password = String::new();
    let mut index: i32;
    for _ in 0..=pw_len {
        index = rand::thread_rng().gen_range(0..=100);
        if index % 2 == 0 {
            password += digits[index as usize % digits.len()].to_string().as_str();
        }
        password += lowercase_chars[index as usize % lowercase_chars.len()].to_string().as_str();

        index = rand::thread_rng().gen_range(0..=100);
        if index % 5 == 0 {
            password += symbols[index as usize % symbols.len()].to_string().as_str();
        }
        password += uppercase_chars[index as usize % uppercase_chars.len()].to_string().as_str();
    }

    println!("Your password is: {}", password);
}
