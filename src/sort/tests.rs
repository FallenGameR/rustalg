macro_rules! generate_sort_tests {
    ($sortImplementation:ident) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use std::fmt::Display;
            use $crate::sort::Sort;

            // Sorting cases
            #[test] fn general_case()      { run(vec![5, 3, 6, 2, 10, 1, 4, 9, 8, 7]); }
            #[test] fn empty()             { let v: Vec<i32> = vec![]; run(v); }
            #[test] fn single_element()    { run(vec![42]); }
            #[test] fn already_sorted()    { run(vec![1, 2, 3, 4, 5]); }
            #[test] fn reverse_sorted()    { run(vec![5, 4, 3, 2, 1]); }
            #[test] fn with_duplicates()   { run(vec![3, 1, 2, 3, 1, 2]); }
            #[test] fn all_identical()     { run(vec![7, 7, 7, 7, 7]); }
            #[test] fn negative_numbers()  { run(vec![-1, -3, -2, 0, 2, 1]); }

            // Different types
            #[test] fn floats()            { run(vec![3.1, 2.4, 5.6, 1.2, 4.8]); }
            #[test] fn strings()           { run(vec!["banana", "apple", "cherry", "date"]); }
            #[test] fn mixed_case_strings(){ run(vec!["banana", "Apple", "cherry", "Date"]); }
            #[test] fn unicode_strings()   { run(vec!["éclair", "apple", "banana", "Äpfel"]); }

            // Larger datasets
            #[test] fn large_dataset()     { run((0..1000).rev().collect::<Vec<i32>>()); }

            fn run<T: PartialOrd + Display>(data: Vec<T>) {
                let mut sorter = $sortImplementation::new(data);
                sorter.sort();
                assert!(sorter.is_sorted());
            }
        }
    };
}

// Visibility re-export to make `super::test_macros::generate_sort_tests` available
pub(crate) use generate_sort_tests;
