use std::iter::FromIterator;

#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

#[derive(Debug, Default)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            next: None
        }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
       SimpleLinkedList {
           head: None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut length = 0;
        let mut node = &self.head;

        loop {
            match node {
                Some(n) => {
                    length += 1;
                    node = &n.next;
                }
                None => break,
            }
        }

        length
    }

    pub fn push(&mut self, _element: T) {
        self.head = Some(Box::new(Node {
            data: _element,
            next: self.head.take()
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.head.take();

        match head {
            Some(mut n) => {
                self.head = n.next.take();

                return Some(n.data);
            }
            None => return None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(n) => Some(&n.data),
            None => None,
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut node = self;
        let mut l = SimpleLinkedList::new();

        loop {
            match node.pop() {
                Some(n) => l.push(n),
                None => break,
            }
        }

        l
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
         let mut list = SimpleLinkedList::new();
         for item in _iter {
             list.push(item);
         }
         list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut v = Vec::new();
        let mut n = self.rev();

        loop {
            match n.pop() {
                Some(n) => v.push(n),
                None => break, 
            }
        }

        v
    }
}
