use super::Sorter;

pub struct InsertionSort {
    binary_search: bool,
}

impl InsertionSort {
    pub fn new(binary_search: bool) -> Self {
        InsertionSort { binary_search }
    }
}

impl Sorter for InsertionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 1..slice.len() {
            if self.binary_search {
                let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    Ok(i) => i,
                    Err(i) => i,
                };
                slice[i..=unsorted].rotate_right(1)
            } else {
                // start at 1 because a list of length 1 is always sorted (only start sorting after first el)
                // slice[unsorted..] is not sorted
                // take slice[unsorted] and place in sorted location in slice[..=unsorted]
                let mut i = unsorted;
                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    i -= 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{InsertionSort, Sorter};

    use pretty_assertions::assert_eq;

    #[test]
    fn test_insertion_non_binary_search() {
        let mut things = vec![4, 1, 5, 2, 3, 9, 6, 7, 10, 8];
        let sorter = InsertionSort::new(false);
        sorter.sort(&mut things);
        assert_eq!(things, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
    #[test]
    fn test_insertion_binary_search() {
        let mut things = vec![4, 1, 5, 2, 3, 9, 6, 7, 10, 8];
        let sorter = InsertionSort::new(true);

        sorter.sort(&mut things);
        assert_eq!(things, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
