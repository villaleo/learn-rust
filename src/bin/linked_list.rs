#[derive(Clone)]
enum Link<T> {
    None,
    Tail { item: T },
    Link { item: T, next: Box<Link<T>> },
}

#[derive(Clone)]
struct Cursor<T> {
    curr: Link<T>,
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
            Link::Tail { item } => {
                self.curr = Link::None;
                return Some(item);
            }
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
            Self::None => self.as_tail(item),
            Self::Tail { .. } => self.as_link(item),
            Self::Link { next, .. } => next.push(item),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self {
            Self::None => None,
            Self::Tail { item } => {
                let item = *item;
                self.as_none();
                return Some(item);
            }
            Self::Link { item, next } => {
                let mut temp = Box::new(Self::None);
                let item = *item;

                std::mem::swap(next, &mut temp);
                self.as_next(*temp);
                return Some(item);
            }
        }
    }

    fn as_tail(&mut self, it: T) {
        *self = match self {
            Self::None => Self::Tail { item: it },
            Self::Link { item: _, next: _ } => Self::Tail { item: it },
            _ => panic!("Illegal state: Cannot convert to Tail")
        }
    }

    fn as_link(&mut self, it: T) {
        *self = match self {
            Self::Tail { item } => Self::Link {
                item: *item,
                next: Box::new(Self::Tail { item: it }),
            },
            _ => panic!("Illegal state: Cannot convert to Link")
        }
    }

    fn as_none(&mut self) {
        *self = std::mem::replace(self, Link::None);
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

    println!("{}", list.pop().unwrap());
    println!("{}", list.pop().unwrap());
    println!("{}", list.pop().unwrap());
}