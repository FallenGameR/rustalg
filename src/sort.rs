use std::fmt::Display;

pub trait Sort<T: PartialOrd + Display> {
    fn get_elements(&self) -> &[T];

    fn sort(&mut self);

    fn exchange(&mut self, i: usize, j: usize);

    fn less(a: &T, b: &T) -> bool {
        a < b
    }

    fn show(&self) {
        for (i, item) in self.get_elements().iter().enumerate() {
            print!("{}: {} ", i, item);
        }
        println!();
    }

    fn is_sorted(&self) -> bool {
        let a = self.get_elements();
        for i in 1..a.len() {
            if Self::less(&a[i], &a[i - 1]) {
                return false;
            }
        }
        true
    }
}