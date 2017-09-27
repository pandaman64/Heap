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
    
    fn test<H: Heap<i32>>(mut heap: H) {
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

    fn test_merge<H: Heap<i32>>(mut h1: H, mut h2: H) {
        h1.add(4);
        h1.add(2);
        h1.add(5);
        h2.add(1);
        h2.add(3);
        h2.add(6);
        h1.merge_in_place(&mut h2);
        
        assert_eq!(h1.remove_min(), Some(1));
        assert_eq!(h1.remove_min(), Some(2));
        assert_eq!(h1.remove_min(), Some(3));
        assert_eq!(h1.remove_min(), Some(4));
        assert_eq!(h1.remove_min(), Some(5));
        assert_eq!(h1.remove_min(), Some(6));
    }

    #[test]
    fn test_binary() {
        let heap = BinaryHeap::<i32>::new();
        test(heap);
    }

    #[test]
    fn test_pairing() {
        let heap = PairingHeap::<i32>::new();
        test(heap);
    }

    #[test]
    fn test_skew() {
        let heap = SkewHeap::<i32>::new();
        test(heap);
    }

    #[test]
    fn test_merge_binary() {
        let h1 = BinaryHeap::<i32>::new();
        let h2 = BinaryHeap::<i32>::new();
        test_merge(h1, h2);
    }

    #[test]
    fn test_merge_pairing() {
        let h1 = PairingHeap::<i32>::new();
        let h2 = PairingHeap::<i32>::new();
        test_merge(h1, h2);
    }

    #[test]
    fn test_merge_skew() {
        let h1 = SkewHeap::<i32>::new();
        let h2 = SkewHeap::<i32>::new();
        test_merge(h1, h2);
    }

    #[inline]
    fn bench<H: Heap<i32>, R: Rng>(heap: &mut H, rng: &mut R) {
        for _ in 1..test::black_box(1000) {
            let x = rng.gen();
            if rng.gen() {
                heap.remove_min();
            } else {
                heap.add(x);
            }
        }
    }

    #[inline]
    fn bench_merge<H: Heap<i32>, R: Rng>(mut h1: H, mut h2: H, rng: &mut R) {
        for _ in 1..test::black_box(10) {
            for _ in 1..test::black_box(100) {
                let x = rng.gen();
                if rng.gen() {
                    h1.add(x);
                } else {
                    h2.add(x);
                }
                if rng.gen() {
                    if rng.gen() {
                        h1.remove_min();
                    } else {
                        h2.remove_min();
                    }
                }
            }
            if rng.gen() {
                h1.merge_in_place(&mut h2);
            } else {
                h2.merge_in_place(&mut h1);
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

    #[bench]
    fn bench_merge_binary(b: &mut Bencher) {
        let mut rng = rand::IsaacRng::new_unseeded();

        b.iter(||{ 
            let h1 = BinaryHeap::<i32>::new();
            let h2 = BinaryHeap::<i32>::new();
            bench_merge(h1, h2, &mut rng)
        });
    }

    #[bench]
    fn bench_merge_pairing(b: &mut Bencher) {
        let mut rng = rand::IsaacRng::new_unseeded();

        b.iter(||{ 
            let h1 = PairingHeap::<i32>::new();
            let h2 = PairingHeap::<i32>::new();
            bench_merge(h1, h2, &mut rng)
        });
    }

    #[bench]
    fn bench_merge_skew(b: &mut Bencher) {
        let mut rng = rand::IsaacRng::new_unseeded();

        b.iter(||{ 
            let h1 = SkewHeap::<i32>::new();
            let h2 = SkewHeap::<i32>::new();
            bench_merge(h1, h2, &mut rng)
        });
    }
}

fn main() {
}
