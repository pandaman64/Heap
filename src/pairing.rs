use std;
use heap::Heap;

pub struct PairingTree<T> {
    top: T,
    subheaps: Vec<PairingTree<T>>
}

impl<T: Ord> PairingTree<T> {
    fn merge_in_place(&mut self, mut other: Self) {
        if self.top > other.top {
            std::mem::swap(self, &mut other);
        }
        self.subheaps.push(other);
    }
}

pub enum PairingHeap<T> {
    Empty,
    Tree(PairingTree<T>)
}

impl<T: Clone + Ord> PairingHeap<T> {
    pub fn new() -> Self {
        PairingHeap::Empty
    }

    fn single(v: T) -> Self {
        PairingHeap::Tree(PairingTree {
            top: v,
            subheaps: vec![]
        })
    }

    fn merge(mut self, other: Self) -> Self {
        self.merge_in_place(other);
        self
    }

    fn merge_in_place(&mut self, mut other: Self) {
        use self::PairingHeap::*;
        use std::mem::swap;
        match *self {
            Empty => swap(self, &mut other),
            Tree(ref mut tree) => 
                match other {
                    Empty => return,
                    Tree(other) => tree.merge_in_place(other)
                }
        }
    }

    fn merge_pairs<I: Iterator<Item=PairingTree<T>>>(mut iter: I) -> PairingHeap<T> {
        use self::PairingHeap::*;
        match iter.next() {
            None => Empty,
            Some(mut first) =>
                match iter.next() {
                    None => Tree(first),
                    Some(second) => {
                        first.merge_in_place(second);
                        Tree(first).merge(Self::merge_pairs(iter))
                    }
                }
        }
    }
}

impl<T: Ord + Clone> Heap<T> for PairingHeap<T> {
    fn add(&mut self, v: T) {
       self.merge_in_place(PairingHeap::single(v));
    }

    fn empty(&self) -> bool {
        use self::PairingHeap::*;
        match *self {
            Empty => true,
            _ => false
        }
    }

    fn get_min(&self) -> Option<T> {
        use self::PairingHeap::*;
        match *self {
            Empty => None,
            Tree(ref tree) => Some(tree.top.clone())
        }
    }

    fn remove_min(&mut self) -> Option<T> {
        use self::PairingHeap::*;
        use std::mem::replace;
        match replace(self, Empty) {
            Empty => None,
            Tree(tree) => {
                *self = Self::merge_pairs(tree.subheaps.into_iter());
                Some(tree.top)
            }
        }
    }
}

