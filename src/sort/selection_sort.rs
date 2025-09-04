use super::Sort;

use std::fmt::Display;

pub struct SelectionSort<T> {
    data: Vec<T>,
}

impl<T> SelectionSort<T> {
    pub fn new(data: Vec<T>) -> Self {
        SelectionSort { data }
    }
}

impl<T> Sort for SelectionSort<T>
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
        let data = &mut self.data;

        for i in 0..data.len() {
            let mut min = i;

            for j in i+1..data.len() {
                if data[j] < data[min] {
                    min = j;
                }
            }

            data.swap(i, min);
        }
    }
}

// --------------------------------------------------------------------/ tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_works_general_case() {
        let mut sorter = SelectionSort::new(vec![5, 3, 6, 2, 10, 1, 4, 9, 8, 7]);
        sorter.sort();
        assert!(sorter.is_sorted());
    }

    #[test]
    fn sort_works_empty() {
        let mut sorter: SelectionSort<i32> = SelectionSort::new(vec![]);
        sorter.sort();
        assert!(sorter.is_sorted());
    }

    #[test]
    fn sort_works_single_element() {
        let mut sorter = SelectionSort::new(vec![42]);
        sorter.sort();
        assert!(sorter.is_sorted());
    }

    #[test]
    fn sort_works_already_sorted() {
        let mut sorter = SelectionSort::new(vec![1, 2, 3, 4, 5]);
        sorter.sort();
        assert!(sorter.is_sorted());
    }

    #[test]
    fn sort_works_reverse_sorted() {
        let mut sorter = SelectionSort::new(vec![5, 4, 3, 2, 1]);
        sorter.sort();
        assert!(sorter.is_sorted());
    }

    #[test]
    fn sort_works_with_duplicates() {
        let mut sorter = SelectionSort::new(vec![3, 1, 2, 3, 1, 2]);
        sorter.sort();
        assert!(sorter.is_sorted());
    }

    #[test]
    fn sort_works_all_identical() {
        let mut sorter = SelectionSort::new(vec![7, 7, 7, 7, 7]);
        sorter.sort();
        assert!(sorter.is_sorted());
    }

    #[test]
    fn sort_works_negative_numbers() {
        let mut sorter = SelectionSort::new(vec![-1, -3, -2, 0, 2, 1]);
        sorter.sort();
        assert!(sorter.is_sorted());
    }

    #[test]
    fn sort_works_floats() {
        let mut sorter = SelectionSort::new(vec![3.1, 2.4, 5.6, 1.2, 4.8]);
        sorter.sort();
        assert!(sorter.is_sorted());
    }

    #[test]
    fn sort_works_strings() {
        let mut sorter = SelectionSort::new(vec!["banana", "apple", "cherry", "date"]);
        sorter.sort();
        assert!(sorter.is_sorted());
    }

    #[test]
    fn sort_works_mixed_case_strings() {
        let mut sorter = SelectionSort::new(vec!["banana", "Apple", "cherry", "Date"]);
        sorter.sort();
        assert!(sorter.is_sorted());
    }

    #[test]
    fn sort_works_unicode_strings() {
        let mut sorter = SelectionSort::new(vec!["éclair", "apple", "banana", "Äpfel"]);
        sorter.sort();
        assert!(sorter.is_sorted());
    }

    #[test]
    fn sort_works_large_dataset() {
        let data: Vec<i32> = (0..1000).rev().collect();
        let mut sorter = SelectionSort::new(data);
        sorter.sort();
        assert!(sorter.is_sorted());
    }
}