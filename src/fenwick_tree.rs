use std::{collections::HashMap, hash::Hash, ops::Index, sync::Arc};

pub struct FenwickTree<T> {
    indexes: HashMap<T, usize>,
    values: Vec<i32>,
    sum: Vec<i32>,
}

impl<T> FenwickTree<T> {
    pub fn new(freq: Vec<(T, i32)>) -> Self
    where
        T: Eq + Hash,
    {
        let capacity = freq.len();

        let mut tree = FenwickTree::<T> {
            indexes: HashMap::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
            sum: vec![0; capacity],
        };

        let mut i = 1;
        for (value, frequency) in freq {
            tree.values.push(frequency);
            tree.indexes.insert(value, i);
            i += 1;
        }

        tree.update();

        // DEBUG
        // println!("values: {:?}", tree.values);
        // println!("sum: {:?}", tree.sum);

        tree
    }

    fn update(&mut self) {
        for i in 0..self.values.len() {
            let i = i + 1;
            // last set bit
            let lsb = i as i32 & -(i as i32);
            let parent = i - lsb as usize;

            let mut sum = 0;
            if parent == i {
                sum = self.values[i - 1];
            } else {
                for j in parent..i {
                    sum += self.values[j];
                }
            }

            self.sum[i - 1] = sum;
        }
    }

    pub fn sum(&self, index: usize) -> i32 {
        let mut i = index + 1;

        let mut sum = 0;
        while i > 0 {
            let lsb = i as i32 & -(i as i32);
            let parent = i - lsb as usize;
            sum += self.sum[i - 1];

            if parent == i {
                i -= 1;
            } else {
                i = parent;
            }
        }

        sum
    }

    pub fn add_count(&mut self, index: T)
    where
        T: Eq + Hash,
    {
        let index = self.indexes[&index] - 1;
        self.values[index] += 1;
        self.update();
    }

    pub fn get_range(&self, index: T) -> (i32, i32)
    where
        T: Eq + Hash,
    {
        let index = self.indexes[&index] - 1;
        let low = if index == 0 { 0 } else { self.sum(index - 1) };
        let high = self.sum(index);

        (low, high)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn fenwick_tree_sum() {
        let freq = vec![('A', 1), ('B', 2), ('C', 3), ('D', 4)];
        // let freq = vec![('A', 5), ('B', 2), ('C', 9), ('D', -3), ('E', 5)];
        let tree = FenwickTree::new(freq);

        assert_eq!(10, tree.sum(3));
        assert_eq!(6, tree.sum(2));
        assert_eq!(3, tree.sum(1));
        assert_eq!(1, tree.sum(0));
    }

    #[test]
    fn fenwick_tree_get_range() {
        let freq = vec![('A', 1), ('B', 2), ('C', 3), ('D', 4)];
        let tree = FenwickTree::new(freq);

        assert_eq!((0, 1), tree.get_range('A'));
        assert_eq!((1, 3), tree.get_range('B'));
        assert_eq!((3, 6), tree.get_range('C'));
        assert_eq!((6, 10), tree.get_range('D'));
    }

    #[test]
    fn fenwick_tree_update() {
        let freq = vec![('A', 1), ('B', 2), ('C', 3), ('D', 4)];
        let mut tree = FenwickTree::new(freq);

        tree.add_count('A');
        assert_eq!((0, 2), tree.get_range('A'));
        assert_eq!((2, 4), tree.get_range('B'));
        assert_eq!((4, 7), tree.get_range('C'));
        assert_eq!((7, 11), tree.get_range('D'));

        tree.add_count('B');
        assert_eq!((0, 2), tree.get_range('A'));
        assert_eq!((2, 5), tree.get_range('B'));
        assert_eq!((5, 8), tree.get_range('C'));
        assert_eq!((8, 12), tree.get_range('D'));
    }
}
