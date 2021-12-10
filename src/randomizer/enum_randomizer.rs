use super::Rng;

pub struct EnumRandomizer<E: Clone> {
    rng: Rng,
    values: Vec<E>
}

impl<E: Clone> EnumRandomizer<E> {
    pub fn new(enum_values: &[E]) -> EnumRandomizer<E> {
        EnumRandomizer {
            rng: Rng::new(),
            values: enum_values.to_vec()
        }
    }

    pub fn random(&mut self) -> E {
        self.rng.from_range(&self.values).clone()
    }

    fn num_from_range(&mut self) -> i32 {
        if self.values.len() == 0 { 0 } else { self.rng.i32_range(0, self.values.len() as i32 - 1) }
    }

    pub fn random_group(&mut self, count: usize) -> Vec<E> {
        use std::collections::HashSet;
        debug_assert!(count <= self.values.len());
        let mut used_indexes: HashSet<i32> = HashSet::new(); 
        for _ in 0..count {
            let mut num = self.num_from_range();
            while used_indexes.contains(&num) {
                num = self.num_from_range();
            }
            used_indexes.insert(num);
        }
        used_indexes.into_iter().map(|idx| self.values[idx as usize].clone()).collect()
    }
}