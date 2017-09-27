pub trait Heap<T> {
    fn new() -> Self where Self: Sized;
    fn add(&mut self, v: T);
    fn empty(&self) -> bool;
    fn get_min(&self) -> Option<T>;
    fn remove_min(&mut self) -> Option<T>;
    fn merge_in_place(&mut self, mut other: Self) where Self: Sized {
        while let Some(v) = other.remove_min() {
            self.add(v)
        }
    }
}

