use super::Sort;
use std::fmt::Display;
use crate::sort::test_macros::generate_sort_tests;

pub struct InsertionSort<T> {
    data: Vec<T>,
}

impl<T> InsertionSort<T> {
    pub fn new(data: Vec<T>) -> Self {
        InsertionSort { data }
    }
}

impl<T> Sort for InsertionSort<T>
where
    T: PartialOrd + Display,
{
    type Item = T;

    fn elements(&self) -> &[T] {
        &self.data
    }

    fn elements_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    fn sort(&mut self) {
        for i in 1..self.elements().len() {
            for j in (1..=i).rev() {
                if self.less(j, j - 1) {
                    self.exchange(j, j - 1);
                }
            }
        }
    }
}

// --------------------------------------------------------------------/ tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_works_general_case() {
        let data = vec![5, 3, 6, 2, 10, 1, 4, 9, 8, 7];
        assert_insertion_sort_works(data);
    }

    #[test]
    fn sort_works_empty() {
        let data: Vec<i32> = vec![];
        assert_insertion_sort_works(data);
    }

    #[test]
    fn sort_works_single_element() {
        let data = vec![42];
        assert_insertion_sort_works(data);
    }

    #[test]
    fn sort_works_already_sorted() {
        let data = vec![1, 2, 3, 4, 5];
        assert_insertion_sort_works(data);
    }

    #[test]
    fn sort_works_reverse_sorted() {
        let data = vec![5, 4, 3, 2, 1];
        assert_insertion_sort_works(data);
    }

    #[test]
    fn sort_works_with_duplicates() {
        let data = vec![3, 1, 2, 3, 1, 2];
        assert_insertion_sort_works(data);
    }

    #[test]
    fn sort_works_all_identical() {
        let data = vec![7, 7, 7, 7, 7];
        assert_insertion_sort_works(data);
    }

    #[test]
    fn sort_works_negative_numbers() {
        let data = vec![-1, -3, -2, 0, 2, 1];
        assert_insertion_sort_works(data);
    }

    #[test]
    fn sort_works_floats() {
        let data = vec![3.1, 2.4, 5.6, 1.2, 4.8];
        assert_insertion_sort_works(data);
    }

    #[test]
    fn sort_works_strings() {
        let data = vec!["banana", "apple", "cherry", "date"];
        assert_insertion_sort_works(data);
    }

    #[test]
    fn sort_works_mixed_case_strings() {
        let data = vec!["banana", "Apple", "cherry", "Date"];
        assert_insertion_sort_works(data);
    }

    #[test]
    fn sort_works_unicode_strings() {
        let data = vec!["éclair", "apple", "banana", "Äpfel"];
        assert_insertion_sort_works(data);
    }

    #[test]
    fn sort_works_large_dataset() {
        let data: Vec<i32> = (0..1000).rev().collect();
        assert_insertion_sort_works(data);
    }

    fn assert_insertion_sort_works<T: PartialOrd + Display>(data: Vec<T>) {
        let mut sorter = InsertionSort::new(data);
        sorter.sort();
        assert!(sorter.is_sorted());
    }
}

generate_sort_tests!(InsertionSort);