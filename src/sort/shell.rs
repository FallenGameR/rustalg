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
        for i in 1..self.elements().len() {
            for j in (1..=i).rev() {
                if !self.less(j, j - 1) {
                    break;
                }

                self.exchange(j, j - 1);
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

        // insertion sort
        // for (int j = i; j > 0 && less(j, j - 1); j--) {
        //     exchange(j, j - 1);

        n = 16

        h = 13
        i = 13 14 15
        j = 13

        h = 4
        i = 4 5 6 7 8 9 10 11 12 13 14 15
        j = rev i..4

        h =  1
        i = 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15
        j = rev i..1















     */
}

// --------------------------------------------------------------------/ tests
#[cfg(test)]
super::tests::generate_sort_tests!(ShellSort);
