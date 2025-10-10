//------------------------------------------------------/ Submodules
pub mod args;
pub mod selection;
pub mod insertion;
pub mod shell;

#[cfg(test)]
mod tests;

//------------------------------------------------------/ Imports
use std::fmt::Display;
use crate::sort::{
    insertion::InsertionSort,
    selection::SelectionSort,
    shell::ShellSort
};

//------------------------------------------------------/ Traits
pub trait Sort {
    type Item: PartialOrd + Display;

    fn elements(&self) -> &[Self::Item];

    fn elements_mut(&mut self) -> &mut [Self::Item];

    fn sort(&mut self);

    fn exchange(&mut self, i: usize, j: usize) {
        self.elements_mut().swap(i, j);
    }

    fn less(&self, i: usize, j: usize) -> bool {
        self.elements()[i] < self.elements()[j]
    }

    fn show(&self) {
        for (i, item) in self.elements().iter().enumerate() {
            print!("{}: {} ", i, item);
        }
        println!();
    }

    fn is_sorted(&self) -> bool {
        for i in 1..self.elements().len() {
            if self.less(i, i - 1) {
                return false;
            }
        }
        true
    }
}

//------------------------------------------------------/ Enums
pub enum Sorter<T> {
    Selection(SelectionSort<T>),
    Insertion(InsertionSort<T>),
    Shell(ShellSort<T>),
}

impl<T> Sorter<T> {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Selection(_) => "selection",
            Self::Insertion(_) => "insertion",
            Self::Shell(_) => "shell",
        }
    }

    pub fn from_algorithm(alg: &args::Algorithm, data: Vec<T>) -> Self {
        match alg {
            args::Algorithm::Selection => Self::selection(data),
            args::Algorithm::Insertion => Self::insertion(data),
            args::Algorithm::Shell => Self::shell(data),
        }
    }

    pub fn selection(data: Vec<T>) -> Self {
        Self::Selection(SelectionSort::new(data))
    }

    pub fn insertion(data: Vec<T>) -> Self {
        Self::Insertion(InsertionSort::new(data))
    }

    pub fn shell(data: Vec<T>) -> Self {
        Self::Shell(ShellSort::new(data))
    }
}

impl<T> Sort for Sorter<T>
where
    T: PartialOrd + Display,
{
    type Item = T;

    fn elements(&self) -> &[Self::Item] {
        match self {
            Self::Selection(s) => s.elements(),
            Self::Insertion(s) => s.elements(),
            Self::Shell(s) => s.elements(),
        }
    }

    fn elements_mut(&mut self) -> &mut [Self::Item] {
        match self {
            Self::Selection(s) => s.elements_mut(),
            Self::Insertion(s) => s.elements_mut(),
            Self::Shell(s) => s.elements_mut(),
        }
    }

    fn sort(&mut self) {
        match self {
            Self::Selection(s) => s.sort(),
            Self::Insertion(s) => s.sort(),
            Self::Shell(s) => s.sort(),
        }
    }
}
