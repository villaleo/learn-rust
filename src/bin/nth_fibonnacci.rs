use std::io::stdin;

fn fibonacci(n: i64) -> i64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}

fn main() {
    let mut num = String::new();
    loop {
        println!("Enter a number: ");
        stdin().read_line(&mut num).expect("Error reading input");
        if num.trim().parse::<i64>().is_err() {
            println!("Error parsing as i64");
            continue;
        }
        break;
    }

    let num = num.trim().parse::<i64>().unwrap();
    println!("The {}th Fibonacci number is {}", num, fibonacci(num));
}
