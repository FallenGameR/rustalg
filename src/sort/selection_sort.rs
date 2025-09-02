use super::Sort;

use std::fmt::Display;

pub struct SelectionSort<T: PartialOrd + Display> {
    data: Vec<T>,
}

impl<T: PartialOrd + Display> SelectionSort<T> {
    pub fn new(data: Vec<T>) -> Self {
        SelectionSort { data }
    }
}

impl<T: PartialOrd + Display> Sort<T> for SelectionSort<T> {
    fn elements(&self) -> &[T] {
        &self.data
    }

    fn elements_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    fn sort(&mut self) {
        !todo!("Implement selection sort algorithm");
    }
}