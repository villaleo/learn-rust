use std::fmt::{Display, Formatter};
use std::io::stdin;

struct Person {
    name: String,
    siblings: Vec<Box<Person>>,
}

fn main() {
    let mut people: Vec<Person> = Vec::from([Person::new(
        "Leonardo".to_string(),
        vec![
            Box::new(Person::new("Ruben".to_string(), vec![])),
            Box::new(Person::new("Santiago".to_string(), vec![])),
        ],
    )]);

    println!("Current list of people:");
    for person in &people {
        println!("\t{}", *person)
    }

    people.push(Person::from_stdin());
    println!("After adding a person:");
    for person in &people {
        println!("\t{}", *person)
    }
}

impl Person {
    pub fn new(name: String, siblings: Vec<Box<Person>>) -> Self {
        Self { name, siblings }
    }

    pub fn from_stdin() -> Self {
        println!("Enter person <name, siblings...>");
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("Error reading line");

        let mut tokens = buffer.trim().split(",").collect::<Vec<&str>>();
        let name = tokens.get(0).unwrap().to_string();
        tokens.remove(0);

        let mut siblings: Vec<Box<Person>> = vec![];
        for entry in tokens {
            siblings.push(Box::new(Person::new(entry.trim().to_string(), vec![])));
        }

        Self { name, siblings }
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let names = &self
            .siblings
            .iter()
            .map(|person: &Box<Person>| person.name.clone())
            .collect::<Vec<String>>();
        write!(f, "{} with siblings {:?}", self.name, names)
    }
}
