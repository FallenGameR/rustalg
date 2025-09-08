//------------------------------------------------------/ Submodules
pub mod selection;
pub mod insertion;

#[cfg(test)]
mod tests;

//------------------------------------------------------/ Re-exports
pub use selection::*;
pub use insertion::*;

//------------------------------------------------------/ Traits
use std::fmt::Display;

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