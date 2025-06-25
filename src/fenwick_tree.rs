use std::{collections::HashMap, hash::Hash};

pub struct FenwickTree<T> {
    indexes: HashMap<T, usize>,
    values: Vec<isize>,
    sum: Vec<isize>,
}

impl<T> FenwickTree<T> {
    pub fn new(freq: Vec<(T, isize)>) -> Self
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
            let lsb = i as isize & -(i as isize);
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

    pub fn sum(&self, index: usize) -> isize {
        let mut i = index + 1;

        let mut sum = 0;
        while i > 0 {
            let lsb = i as isize & -(i as isize);
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

    pub fn total_sum(&self) -> isize {
        let i = self.sum.len() - 1;
        self.sum(i)
    }

    pub fn add_count(&mut self, index: T)
    where
        T: Eq + Hash,
    {
        let index = self.indexes[&index] - 1;
        self.values[index] += 1;
        self.update();
    }

    pub fn reduce_count(&mut self, index: T)
    where
        T: Eq + Hash,
    {
        let index = self.indexes[&index] - 1;
        self.values[index] -= 1;
        self.update();
    }

    pub fn get_bounds(&self, index: T) -> (isize, isize)
    where
        T: Eq + Hash,
    {
        let index = self.indexes[&index] - 1;
        let low = if index == 0 { 0 } else { self.sum(index - 1) };
        let high = self.sum(index);

        (low, high)
    }

    pub fn search_range(&self, range: isize) -> Option<T>
    where
        T: Eq + Hash + Copy,
    {
        for (value, _) in &self.indexes {
            let (low, high) = self.get_bounds(*value);

            if low <= range && range < high {
                return Some(*value);
            }
        }

        None
    }

    pub fn get_total_count(&self) -> isize {
        self.sum(self.sum.len() - 1)
    }

    pub fn len(&self) -> usize {
        self.values.len()
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
    fn fenwick_tree_get_bounds() {
        let freq = vec![('A', 1), ('B', 2), ('C', 3), ('D', 4)];
        let tree = FenwickTree::new(freq);

        assert_eq!((0, 1), tree.get_bounds('A'));
        assert_eq!((1, 3), tree.get_bounds('B'));
        assert_eq!((3, 6), tree.get_bounds('C'));
        assert_eq!((6, 10), tree.get_bounds('D'));
    }

    #[test]
    fn fenwick_tree_update() {
        let freq = vec![('A', 1), ('B', 2), ('C', 3), ('D', 4)];
        let mut tree = FenwickTree::new(freq);

        tree.add_count('A');
        assert_eq!((0, 2), tree.get_bounds('A'));
        assert_eq!((2, 4), tree.get_bounds('B'));
        assert_eq!((4, 7), tree.get_bounds('C'));
        assert_eq!((7, 11), tree.get_bounds('D'));

        tree.add_count('B');
        assert_eq!((0, 2), tree.get_bounds('A'));
        assert_eq!((2, 5), tree.get_bounds('B'));
        assert_eq!((5, 8), tree.get_bounds('C'));
        assert_eq!((8, 12), tree.get_bounds('D'));
    }

    #[test]
    fn fenwick_tree_search_range() {
        let freq = vec![('A', 1), ('B', 2), ('C', 3), ('D', 4)];
        let tree = FenwickTree::new(freq);

        assert_eq!(Some('A'), tree.search_range(0));

        assert_eq!(Some('B'), tree.search_range(1));
        assert_eq!(Some('B'), tree.search_range(2));

        assert_eq!(Some('C'), tree.search_range(3));
        assert_eq!(Some('C'), tree.search_range(4));
        assert_eq!(Some('C'), tree.search_range(5));
        assert_eq!(Some('D'), tree.search_range(6));
        assert_eq!(Some('D'), tree.search_range(7));
        assert_eq!(Some('D'), tree.search_range(8));
        assert_eq!(Some('D'), tree.search_range(9));
    }
}
