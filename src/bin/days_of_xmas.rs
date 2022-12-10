use std::collections::HashMap;
use std::io::stdin;

fn main() {
    let chorus = |i: &str| println!("On the {} day of Christmas my true love sent to me", i);
    let days_spelled_out = HashMap::<u8, &str>::from(
        [
            (1, "first"), (2, "second"), (3, "third"), (4, "fourth"), (5, "fifth"), (6, "sixth"),
            (7, "seventh"), (8, "eighth"), (9, "ninth"), (10, "tenth"), (11, "eleventh"), (12, "twelfth")
        ]
    );

    let mut num = String::new();
    loop {
        println!("Enter a number (1-12):");
        stdin().read_line(&mut num).expect("Error reading num");
        if num.trim().parse::<u8>().is_err() {
            continue;
        }
        break;
    }

    let num = num.trim().parse::<u8>().unwrap();
    for i in 1..=num {
        chorus(days_spelled_out.get(&i).unwrap());
        for j in (1..i + 1).rev() {
            match j {
                1 => println!("\tA partridge in a pear tree"),
                2 => println!("\tTwo turtle-doves"),
                3 => println!("\tThree French hens"),
                4 => println!("\tFour calling birds"),
                5 => println!("\tFive golden rings!"),
                6 => println!("\tSix geese a-laying"),
                7 => println!("\tSeven swans a-swimming"),
                8 => println!("\tEight maids a-milking"),
                9 => println!("\tNine ladies dancing"),
                10 => println!("\tTen lords a-leaping"),
                11 => println!("\tEleven pipers piping"),
                12 => println!("\tTwelve drummers drumming"),
                _ => println!(),
            }
        }
    }
}
