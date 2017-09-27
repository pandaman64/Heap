pub trait Heap<T> {
    fn add(&mut self, v: T);
    fn empty(&self) -> bool;
    fn get_min(&self) -> Option<T>;
    fn remove_min(&mut self) -> Option<T>;
    fn merge_in_place(&mut self, other: &mut Self) where Self: Sized {
        while let Some(v) = other.remove_min() {
            self.add(v)
        }
    }
}

