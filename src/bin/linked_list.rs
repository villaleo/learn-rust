use std::fmt::{Display, Formatter};

/// A linked list [Link].
///
/// Create a new linked list using [Link::new].
#[derive(Clone)]
enum Link<T> {
    /// An empty link (used to represent null).
    None,
    /// A linked list node with a `value` and a reference to the `next` node.
    Link { item: T, next: Box<Link<T>> },
}

/// An iterator over a linked list.
#[derive(Clone)]
struct LinkIterator<T> {
    /// The current node we are iterating over.
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
    type IntoIter = LinkIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkIterator {
            curr: self
        }
    }
}

impl<T> Iterator for LinkIterator<T> where T: Copy {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let next = match self.curr {
            Link::None => None,
            Link::Link { item, ref mut next } => {
                let mut n = Box::new(Link::None);
                std::mem::swap(next, &mut n);
                self.curr = *n;
                Some(item)
            }
        };
        next
    }
}

impl<T> Link<T> where T: Copy + PartialEq {
    /// Creates a new linked list.
    pub fn new() -> Self {
        Self::None
    }

    /// Pushes an item to the back of the list.
    ///
    /// The entire list must be traversed to insert so, the operation is O(n) time.
    pub fn push_back(&mut self, item: T) {
        match self {
            Self::None => self.as_link(item),
            Self::Link { next, .. } => next.push_back(item),
        }
    }

    /// Pops the first item from the list and returns it, wrapped in an [Option]. If the list is empty, [None] is returned.
    ///
    /// `self` is a reference to the first item in the list so, the operation is O(1) time.
    pub fn pop_front(&mut self) -> Option<T> {
        match self {
            Self::None => None,
            Self::Link { item, next } => {
                let mut temp = Box::new(Self::None);
                let item = *item;

                std::mem::swap(next, &mut temp);
                self.as_next(*temp);
                Some(item)
            }
        }
    }

    /// Returns `true` if the item is in the list. `false` otherwise.
    ///
    /// The entire list will be traversed to find `item`, so the operation is O(n) time.
    pub fn contains(&self, item: T) -> bool {
        for node in self.clone() {
            if node == item {
                return true;
            }
        }
        return false;
    }

    /// Transforms `self` to a [Link::Link]. Panics if `self` is already a [Link::Link].
    fn as_link(&mut self, it: T) {
        *self = match self {
            Self::None => {
                Self::Link {
                    item: it,
                    next: Box::new(Self::None),
                }
            }
            Self::Link { .. } => panic!("Illegal state: Cannot convert to Link")
        }
    }

    /// Transforms `self` to the [Link] supplied.
    fn as_next(&mut self, next: Link<T>) {
        *self = next;
    }
}

fn main() {
    let mut list = Link::<i32>::new();
    list.push_back(10);
    list.push_back(20);
    list.push_back(30);
    list.push_back(40);
    list.push_back(50);

    println!("List: {}", list);
    println!("After popping off {}:\n{}", list.pop_front().unwrap(), list);
    println!("List contains 40: {}", list.contains(40));
    println!("List contains 10: {}", list.contains(10));
}
