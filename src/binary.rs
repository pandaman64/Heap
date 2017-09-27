use heap::Heap;

pub struct BinaryHeap<T> {
    buffer: Vec<T>
}

impl<T: Clone + Ord + Default> BinaryHeap<T> {
    pub fn new() -> Self {
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

