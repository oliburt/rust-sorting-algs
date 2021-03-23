use super::Sorter;

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        // [sorted | not sorted]
        for unsorted in 1..slice.len() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion() {
        let mut things = vec![4, 1, 5, 2, 3, 9, 6, 7, 10, 8];
        InsertionSort::sort(&mut things);
        assert_eq!(things, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
