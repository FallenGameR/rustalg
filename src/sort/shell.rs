use super::Sort;
use std::fmt::Display;

pub struct ShellSort<T> {
    data: Vec<T>,
}

// Iterative insertion sort, insertion sort is very fast for partly sorted data, O(inversions + n)
// Inversion is any pair of elements that are out of order
impl<T> ShellSort<T> {
    pub fn new(data: Vec<T>) -> Self {
        ShellSort { data }
    }
}

impl<T> Sort for ShellSort<T>
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
        // Build h sequence, this one is ok in practice:  1, 4, 13, 40, 121, 364, 1093, ...
        let mut h = Vec::new();
        let mut k = 1;
        while k < self.elements().len() / 3 {
            h.push(k);
            k = 3 * k + 1;
        }

        // Sort with decreasing h
        while let Some(gap) = h.pop() {
            // Insertion sort with gap
            for i in gap..self.elements().len() {
                for j in (gap..=i).rev().step_by(gap) {
                    if !self.less(j, j - gap) {
                        break;
                    }

                    self.exchange(j, j - gap);
                }
            }
        }
    }

    /*
    Reference implementation in Java:

    public static void sort(Comparable[] a) {
        int n = a.length;

        // 3x+1 increment sequence: 1, 4, 13, 40, 121, 364, 1093, ...
        int h = 1;
        while (h < n / 3) {
            h = 3 * h + 1;
        }

        while (h >= 1) { // enumerates the h sequence
            for (int i = h; i < n; i++) { // enumerates the slice sequence, why not start from 0?
                for (int j = i; j >= h && less(a[j], a[j - h]); j -= h) { // insertion sort in current slice
                    exchange(a, j, j - h);
                }
            }
            h = h / 3;
        }
     }
     */
}

// --------------------------------------------------------------------/ tests
#[cfg(test)]
super::tests::generate_sort_tests!(ShellSort);
