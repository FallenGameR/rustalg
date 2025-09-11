use super::Sort;
use std::fmt::Display;

pub struct InsertionSort<T> {
    data: Vec<T>,
}

// Very fast for partly sorted data, O(inversions + n)
// Inversion is any pair of elements that are out of order
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
                else {
                    break;
                }

                // java code for reference:
                // for (int j = i; j > 0 && less(j, j - 1); j--) {
                //     exchange(j, j - 1);

                // less exchanges but requires T: Clone and thus
                // under the hood drop would be called more often
                // than necessary; or can be rewriten in unsafe way
                //
                // if a[i] < a[i - 1] {
                //     let key = a[i].clone();              // save
                //     let mut j = i;
                //     while j > 0 && a[j - 1] > key {
                //         a[j] = a[j - 1].clone();         // shift right
                //         j -= 1;
                //     }
                //     a[j] = key;                          // insert
                // }
            }
        }
    }
}

// --------------------------------------------------------------------/ tests
#[cfg(test)]
super::tests::generate_sort_tests!(InsertionSort);
