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
        !todo!("Implement selection sort algorithm");
    }

}