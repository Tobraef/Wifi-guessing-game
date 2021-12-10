use rand::rngs::SmallRng;
use rand::{SeedableRng, RngCore};

pub struct Rng(SmallRng);

impl Rng {
    pub fn new() -> Rng {
        Rng(SmallRng::from_entropy())
    }
    
    pub fn i32_range(&mut self, low_bound: i32, high_bound: i32) -> i32 {
        debug_assert!(low_bound <= high_bound, format!("low: {}, high: {}", low_bound, high_bound));
        if low_bound == high_bound {
            return low_bound;
        }
        ((self.0.next_u32() as i32).abs() % (high_bound - low_bound + 1)) + low_bound
    }

    pub fn from_range<'a, T>(&mut self, range: &'a [T]) -> &'a T {
        &range[self.i32_range(0, range.len() as i32 - 1) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_from_range_i32() {
        use std::collections::HashSet;

        let mut rng = Rng::new();
        let mut rolled: HashSet<i32> = HashSet::new();
        let low_bound = -2;
        let high_bound = 6;
        for _ in 0..1_000 {
            rolled.insert(rng.i32_range(low_bound, high_bound));
        }
        for i in low_bound..high_bound {
            assert!(rolled.contains(&i), "Didn't find {}", i);
        }
        assert_eq!(rolled.len() as i32, high_bound - low_bound + 1);
    }

    #[test]
    fn random_from_range() {
        use std::collections::HashMap;
        use std::iter::FromIterator;

        let iters = 10_000;
        let mut rng = Rng::new();
        let range = vec![1, 2, 3, 4, 5];
        let mut frequencies: HashMap<i32, i32> = HashMap::from_iter(range.iter().map(|x| (*x, 0)));
        for _ in 0..iters {
            frequencies.entry(*rng.from_range(&range)).and_modify(|v| *v += 1);
        }
        let avg = iters / range.len() as i32;
        for (_k, v) in &frequencies {
            assert!((*v - avg).abs() < iters / 100, "Average above expected, received: {}, expected: {}", *v, avg);
        }
    }
}