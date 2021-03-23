use super::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 1..slice.len() {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    swapped = true;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BubbleSort, Sorter};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_bubble() {
        let mut things = vec![4, 1, 5, 2, 3];
        BubbleSort::sort(&mut things);
        assert_eq!(things, vec![1, 2, 3, 4, 5]);
    }
}
