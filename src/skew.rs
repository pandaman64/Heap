use heap::Heap;

pub struct SkewTree<T> {
    value: T,
    left: Box<SkewHeap<T>>,
    right: Box<SkewHeap<T>>,
}

pub enum SkewHeap<T> {
    Empty,
    Tree(SkewTree<T>),
}

impl<T: Ord> SkewHeap<T> {
    pub fn new() -> Self {
        self::SkewHeap::Empty
    }

    fn single(value: T) -> Self {
        use self::SkewHeap::*;
        Tree(SkewTree {
            value: value,
            left: Box::new(Empty),
            right: Box::new(Empty),
        })
    }

    fn merge_in_place(&mut self, mut other: Self) {
        use std::mem::swap;
        use self::SkewHeap::*;
        if let Tree(ref mut this) = *self {
            if let Tree(mut other) = other {
                if this.value > other.value {
                    swap(this, &mut other)
                }
                this.right.merge_in_place(Tree(other));
                swap(&mut this.left, &mut this.right);
            }
        } else {
            swap(self, &mut other)
        }
    }
}

impl<T: Ord + Clone> Heap<T> for SkewHeap<T> {
    fn new() -> Self {
        self::SkewHeap::new()
    }

    fn add(&mut self, value: T) {
        self.merge_in_place(SkewHeap::single(value))
    }

    fn empty(&self) -> bool {
        use self::SkewHeap::*;
        match *self {
            Empty => true,
            _ => false,
        }
    }

    fn get_min(&self) -> Option<T> {
        use self::SkewHeap::*;
        match *self {
            Empty => None,
            Tree(SkewTree { ref value, .. }) => Some(value.clone()),
        }
    }

    fn remove_min(&mut self) -> Option<T> {
        use std::mem::replace;
        use self::SkewHeap::*;

        match replace(self, Empty) {
            Empty => None,
            Tree(SkewTree {
                     value,
                     mut left,
                     right,
                 }) => {
                left.merge_in_place(*right);
                *self = *left;
                Some(value)
            }
        }
    }

    fn merge_in_place(&mut self, other: Self) {
        SkewHeap::merge_in_place(self, other);
    }
}
