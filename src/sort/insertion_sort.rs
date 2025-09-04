use super::Sort;
use std::fmt::Display;

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
        !todo!("Implement insertion sort");
    }
}
