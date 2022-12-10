use std::cmp::Ordering;
use std::io::stdin;
use rand::Rng;

fn main() {
    const LOWER_LIMIT: u32 = 1;
    const UPPER_LIMIT: u32 = 100;

    let secret = rand::thread_rng().gen_range(LOWER_LIMIT..UPPER_LIMIT + 1);
    loop {
        println!("Enter a number from {} to {}:", LOWER_LIMIT, UPPER_LIMIT);
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");
        if guess.trim().parse::<u32>().is_err() { continue; }

        let guess = guess.trim().parse::<u32>().unwrap();
        match guess.cmp(&secret) {
            Ordering::Less => println!("Your guess is too small"),
            Ordering::Equal => {
                println!("You got it! 🎉 Thanks for playing");
                break;
            }
            Ordering::Greater => println!("Your guess is too large!"),
        }
    }
}
