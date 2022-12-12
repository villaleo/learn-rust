use std::fmt::{Display, Formatter};

#[derive(Clone)]
enum Link<T> {
    None,
    Link { item: T, next: Box<Link<T>> },
}

#[derive(Clone)]
struct Cursor<T> {
    curr: Link<T>,
}

impl<T> Display for Link<T> where T: Copy + Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[").unwrap();
        for i in self.clone() {
            write!(f, "{} -> ", i).unwrap();
        }

        write!(f, "]")
    }
}

impl<T> IntoIterator for Link<T> where T: Copy {
    type Item = T;
    type IntoIter = Cursor<T>;

    fn into_iter(self) -> Self::IntoIter {
        Cursor {
            curr: self
        }
    }
}

impl<T> Iterator for Cursor<T> where T: Copy {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let next = match self.curr {
            Link::None => None,
            Link::Link { item, ref mut next } => {
                let mut n = Box::new(Link::None);
                std::mem::swap(next, &mut n);
                self.curr = *n;
                return Some(item);
            }
        };
        return next;
    }
}

impl<T> Link<T> where T: Copy {
    pub fn new() -> Self {
        Self::None
    }

    pub fn push(&mut self, item: T) {
        match self {
            Self::None => self.as_link(item),
            Self::Link { next, .. } => next.push(item),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self {
            Self::None => None,
            Self::Link { item, next } => {
                let mut temp = Box::new(Self::None);
                let item = *item;

                std::mem::swap(next, &mut temp);
                self.as_next(*temp);
                return Some(item);
            }
        }
    }

    fn as_link(&mut self, it: T) {
        *self = match self {
            Self::None => {
                Self::Link {
                    item: it,
                    next: Box::new(Self::None),
                }
            },
            _ => panic!("Illegal state: Cannot convert to Link")
        }
    }

    fn as_next(&mut self, next: Link<T>) {
        *self = next;
    }
}

fn main() {
    let mut list = Link::<i32>::new();
    list.push(10);
    list.push(20);
    list.push(30);
    list.push(40);
    list.push(50);

    println!("{}", list);
    let value = list.pop().unwrap();
    println!("After popping off {}:\n{}", value, list);
}
