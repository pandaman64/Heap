
trait Heap<T> {
    fn add(&mut self, v: T);
    fn empty(&self) -> bool;
    fn get_min(&self) -> Option<T>;
    fn remove_min(&mut self) -> Option<T>;
}

struct BinaryHeap<T> {
    buffer: Vec<T>
}

impl<T: Clone + Ord + Default> BinaryHeap<T> {
    fn new() -> Self {
        BinaryHeap{ buffer: vec![ Default::default() ] }
    }

    fn size(&self) -> usize {
        self.buffer.len() - 1
    }

    fn upheap(&mut self, idx: usize) {
        if idx <= 1 {
            return;
        }
        if self.buffer[idx / 2] > self.buffer[idx] {
            self.buffer.swap(idx / 2, idx);
            self.upheap(idx / 2);
        }
    }

    fn downheap(&mut self, idx: usize) {
        if idx * 2 > self.size() {
            return;
        }
        if idx * 2 + 1 <= self.size() && self.buffer[idx * 2 + 1] < self.buffer[idx * 2] && self.buffer[idx * 2 + 1] < self.buffer[idx] {
            self.buffer.swap(idx * 2 + 1, idx);
            self.downheap(idx * 2 + 1);
        } else if self.buffer[idx * 2] < self.buffer[idx] {
            self.buffer.swap(idx * 2, idx);
            self.downheap(idx * 2);
        }
    }
}

impl<T: Clone + Ord + Default> Heap<T> for BinaryHeap<T> {
    fn add(&mut self, v: T) {
        self.buffer.push(v);
        let idx = self.size();
        self.upheap(idx);
    }

    fn empty(&self) -> bool {
        self.size() == 0
    }

    fn get_min(&self) -> Option<T> {
        if self.empty() {
            None
        } else {
            let idx = self.size();
            Some(self.buffer[idx].clone())
        }
    }

    fn remove_min(&mut self) -> Option<T> {
        if !self.empty() {
            let top = self.buffer.swap_remove(1);
            self.downheap(1);
            Some(top)
        } else {
            None
        }
    }
}

struct ParingTree<T> {
    top: T,
    subheaps: Vec<ParingTree<T>>
}

enum ParingHeap<T> {
    Empty,
    Tree(ParingTree<T>)
}

impl<T: Clone + Ord> ParingHeap<T> {
    fn new() -> Self {
        ParingHeap::Empty
    }

    fn single(v: T) -> Self {
        ParingHeap::Tree(ParingTree {
            top: v,
            subheaps: vec![]
        })
    }

    fn is_empty(&self) -> bool {
        use ParingHeap::*;
        match *self {
            Empty => true,
            _ => false
        }
    }

    fn merge(mut self, other: Self) -> Self {
        self.merge_in_place(other);
        self
    }

    fn merge_in_place(&mut self, mut other: Self) {
        use ParingHeap::*;
        use std::mem::swap;
        match *self {
            Empty => swap(self, &mut other),
            Tree(ref mut tree) => 
                match other {
                    Empty => return,
                    Tree(mut other) => {
                        if tree.top > other.top {
                            swap(tree, &mut other);
                        }
                        tree.subheaps.push(other);
                    }
                }
        }
    }

    fn merge_pairs<I: Iterator<Item=ParingTree<T>>>(mut iter: I) -> ParingHeap<T> {
        use ParingHeap::*;
        match iter.next() {
            None => Empty,
            Some(first) =>
                match iter.next() {
                    None => Tree(first),
                    Some(second) => Tree(first).merge(Tree(second)).merge(Self::merge_pairs(iter))
                }
        }
    }
}

impl<T: Ord + Clone> Heap<T> for ParingHeap<T> {
    fn add(&mut self, v: T) {
       self.merge_in_place(ParingHeap::single(v));
    }

    fn empty(&self) -> bool {
        self.is_empty()
    }

    fn get_min(&self) -> Option<T> {
        use ParingHeap::*;
        match *self {
            Empty => None,
            Tree(ref tree) => Some(tree.top.clone())
        }
    }

    fn remove_min(&mut self) -> Option<T> {
        use ParingHeap::*;
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

fn test(heap: &mut Heap<i32>) {
    heap.add(1);
    heap.add(0);
    heap.add(6);
    heap.add(4);
    heap.add(-6);
    heap.add(5);
    heap.add(3);

    while !heap.empty() {
        println!("{:?}", heap.remove_min());
    }
}

fn main() {
    let mut heap = ParingHeap::new();
    test(&mut heap);    
}
