use std::io::stdin;
use std::collections::VecDeque;

const ASCII_0_OFFSET: u8 = 48u8;

fn main() {
    println!("Text to binary converter\nEnter some text:");
    let mut text = String::new();
    stdin().read_line(&mut text).expect("Could not read text");
    text = text.trim().to_string();

    for digit in text.as_bytes() {
        print!("{} ", to_binary(*digit));
    }
    println!();
}

fn to_binary(mut n: u8) -> String {
    let mut bits = VecDeque::<u8>::new();

    while n != 0 {
        let bit = n % 2;
        bits.push_front(bit);
        n /= 2;
    }

    let mut result = String::new();
    for bit in bits {
        result.push((bit + ASCII_0_OFFSET) as char);
    }
    result
}
