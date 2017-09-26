#![feature(test)]

extern crate test;
extern crate rand;

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

struct PairingTree<T> {
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

enum PairingHeap<T> {
    Empty,
    Tree(PairingTree<T>)
}

impl<T: Clone + Ord> PairingHeap<T> {
    fn new() -> Self {
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
        use PairingHeap::*;
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
        use PairingHeap::*;
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
        use PairingHeap::*;
        match *self {
            Empty => true,
            _ => false
        }
    }

    fn get_min(&self) -> Option<T> {
        use PairingHeap::*;
        match *self {
            Empty => None,
            Tree(ref tree) => Some(tree.top.clone())
        }
    }

    fn remove_min(&mut self) -> Option<T> {
        use PairingHeap::*;
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

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use rand::Rng;

    #[inline]
    fn bench<H: Heap<i32>, R: Rng>(heap: &mut H, rng: &mut R) {
        for x in 1..test::black_box(1000) {
            if rng.gen() {
                heap.remove_min();
            } else {
                heap.add(x);
            }
        }
    }

    #[bench]
    fn bench_binary(b: &mut Bencher) {
        let mut rng = rand::IsaacRng::new_unseeded();
        let mut heap = BinaryHeap::<i32>::new();

        b.iter(|| bench(&mut heap, &mut rng));
    }

    #[bench]
    fn bench_pairing(b: &mut Bencher) {
        let mut rng = rand::IsaacRng::new_unseeded();
        let mut heap = PairingHeap::<i32>::new();

        b.iter(|| bench(&mut heap, &mut rng));
    }
}

fn main() {
    let mut heap = PairingHeap::new();
    test(&mut heap);    
}
