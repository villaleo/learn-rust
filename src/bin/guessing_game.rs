use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn is_guess_correct(n: u32, secret: &u32) -> bool {
    match n.cmp(secret) {
        Ordering::Less => println!("Your guess is too small"),
        Ordering::Equal => {
            println!("You got it! 🎉 Thanks for playing");
            return true;
        }
        Ordering::Greater => println!("Your guess is too large!"),
    }

    return false;
}

fn main() {
    const LOWER_LIMIT: u32 = 1;
    const UPPER_LIMIT: u32 = 100;

    let secret = rand::thread_rng().gen_range(LOWER_LIMIT..UPPER_LIMIT + 1);
    loop {
        println!("Enter a number from {} to {}:", LOWER_LIMIT, UPPER_LIMIT);
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");
        match guess.trim().parse::<u32>() {
            Ok(n) => {
                if is_guess_correct(n, &secret) {
                    break;
                }
            }
            Err(_) => continue,
        }
    }
}
