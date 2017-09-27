pub trait Heap<T> {
    fn add(&mut self, v: T);
    fn empty(&self) -> bool;
    fn get_min(&self) -> Option<T>;
    fn remove_min(&mut self) -> Option<T>;
}

