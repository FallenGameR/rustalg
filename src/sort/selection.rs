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
        for i in 0..self.elements().len() {
            let mut min = i;

            // Find next minimum element in the unsorted part of the array
            for j in i+1..self.elements().len() {
                if self.less(j, min) {
                    min = j;
                }
            }

            // Data movement is minimal in this alg
            self.exchange(i, min);
        }
    }
}


// --------------------------------------------------------------------/ tests
#[cfg(test)]
super::tests::generate_sort_tests!(SelectionSort);

