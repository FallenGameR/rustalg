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

        for i in 1..data.len() {
            let current = &data[i-1];

            for j in i..data.len() {
                let candidate = &data[j];

                if candidate < current {
                    data.swap(i-1, j);
                }
            }
        }
    }
}