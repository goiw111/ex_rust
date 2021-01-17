use std::iter::FromIterator;
use std::rc::Rc;

use crate::SimpleLinkedList::{New,Node};
#[derive(Clone)]
pub enum SimpleLinkedList<T> {
    New,
    Node(Rc<(T,SimpleLinkedList<T>)>),
}

impl<T: Clone + std::cmp::PartialEq> SimpleLinkedList<T> {
    pub fn new() -> Self {
        New
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        match *self {
            Node(ref r)=> 1 + r.1.len(),
            New             => 0
        }
    }

    pub fn push(&mut self, element: T) {
        *self = match self {
            New         =>  Node(Rc::new((element,SimpleLinkedList::new()))),
            Node(ref r) =>  Node(Rc::new((element,Node(r.clone())))),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match *self {
            New         =>  None,
            Node(ref r) =>  {
                let t = r.0.clone();
                *self = match r.1 {
                    New         =>  New,
                    Node(ref r) =>  Node(r.clone()),
                };
                Some(t)
            },
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match *self {
            New         =>  None,
            Node(ref r) =>  Some(&r.0),
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut l = SimpleLinkedList::new();
        let mut r = self.clone();
        while (&r).peek() != None {
            l.push(r.pop().unwrap());
        }
        l
    }
}

impl<T: Clone + PartialEq>  FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut c = SimpleLinkedList::new();
        for i in iter {
            c.push(i);
        }
        c
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

impl<T: Clone + PartialEq> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut l = self.rev().clone();
        let mut v = Vec::new();
        while (&l).peek() != None {
            v.push(l.pop().unwrap());
        }
        v
    }
}
