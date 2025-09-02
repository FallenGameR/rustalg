pub mod selection_sort;

pub use selection_sort::*;

use std::fmt::Display;

pub trait Sort {
    type Item: PartialOrd + Display;

    fn elements(&self) -> &[Self::Item];

    fn elements_mut(&mut self) -> &mut [Self::Item];

    fn sort(&mut self);

    fn exchange(&mut self, i: usize, j: usize) {
        self.elements_mut().swap(i, j);
    }

    fn less(a: &Self::Item, b: &Self::Item) -> bool {
        a < b
    }

    fn show(&self) {
        for (i, item) in self.elements().iter().enumerate() {
            print!("{}: {} ", i, item);
        }
        println!();
    }

    fn is_sorted(&self) -> bool {
        let a = self.elements();
        for i in 1..a.len() {
            if Self::less(&a[i], &a[i - 1]) {
                return false;
            }
        }
        true
    }
}