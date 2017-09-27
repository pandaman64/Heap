#![feature(test)]

extern crate test;
extern crate rand;

mod heap;
mod binary;
mod pairing;
mod skew;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use rand::Rng;
    use heap::Heap;
    use binary::BinaryHeap;
    use pairing::PairingHeap;
    use skew::SkewHeap;
    
    fn test(heap: &mut Heap<i32>) {
        heap.add(1);
        heap.add(0);
        heap.add(6);
        heap.add(4);
        heap.add(-6);
        heap.add(5);
        heap.add(3);

        assert_eq!(heap.remove_min(), Some(-6));
        assert_eq!(heap.remove_min(), Some(0));
        assert_eq!(heap.remove_min(), Some(1));
        assert_eq!(heap.remove_min(), Some(3));
        assert_eq!(heap.remove_min(), Some(4));
        assert_eq!(heap.remove_min(), Some(5));
        assert_eq!(heap.remove_min(), Some(6));
    }

    #[test]
    fn test_binary() {
        let mut heap = BinaryHeap::<i32>::new();
        test(&mut heap);
    }

    #[test]
    fn test_pairing() {
        let mut heap = PairingHeap::<i32>::new();
        test(&mut heap);
    }

    #[test]
    fn test_skew() {
        let mut heap = SkewHeap::<i32>::new();
        test(&mut heap);
    }

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

    #[bench]
    fn bench_skew(b: &mut Bencher) {
        let mut rng = rand::IsaacRng::new_unseeded();
        let mut heap = SkewHeap::<i32>::new();

        b.iter(|| bench(&mut heap, &mut rng));
    }
}

fn main() {
}
