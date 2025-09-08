#[macro_export]
macro_rules! generate_sort_tests {
    ($sort:ident) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use std::fmt::Display;
            use $crate::sort::Sort;

            fn run<T: PartialOrd + Display>(data: Vec<T>) {
                let mut sorter = $sort::new(data);
                sorter.sort();
                assert!(sorter.is_sorted());
            }

            #[test] fn sort_works_general_case()      { run(vec![5, 3, 6, 2, 10, 1, 4, 9, 8, 7]); }
            #[test] fn sort_works_empty()             { let v: Vec<i32> = vec![]; run(v); }
            #[test] fn sort_works_single_element()    { run(vec![42]); }
            #[test] fn sort_works_already_sorted()    { run(vec![1, 2, 3, 4, 5]); }
            #[test] fn sort_works_reverse_sorted()    { run(vec![5, 4, 3, 2, 1]); }
            #[test] fn sort_works_with_duplicates()   { run(vec![3, 1, 2, 3, 1, 2]); }
            #[test] fn sort_works_all_identical()     { run(vec![7, 7, 7, 7, 7]); }
            #[test] fn sort_works_negative_numbers()  { run(vec![-1, -3, -2, 0, 2, 1]); }

            // Separate type groups (floats, &str)
            #[test] fn sort_works_floats()            { run(vec![3.1, 2.4, 5.6, 1.2, 4.8]); }
            #[test] fn sort_works_strings()           { run(vec!["banana", "apple", "cherry", "date"]); }
            #[test] fn sort_works_mixed_case_strings(){ run(vec!["banana", "Apple", "cherry", "Date"]); }
            #[test] fn sort_works_unicode_strings()   { run(vec!["éclair", "apple", "banana", "Äpfel"]); }

            #[test] fn sort_works_large_dataset()     { run((0..1000).rev().collect::<Vec<i32>>()); }
        }
    };
}