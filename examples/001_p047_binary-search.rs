use std::cmp::Ordering::*;

fn main() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    dbg!(array);

    let searched = 4;
    dbg!(searched);

    let index = binary_search(&array, searched);
    dbg!(index);
}

///
/// Performs a binary search on a sorted array to find the index of a given key.
///
/// ### Args
///
/// - `array` - A reference to a sorted array of integers.
/// - `searched` - The integer value to search for in the array.
///
/// ### Returns
///
/// The index of the `key` in the array if found, otherwise -1.
///
fn binary_search(array: &[i32; 5], searched: i32) -> i32 {
    let mut lo = 0;
    let mut hi = array.len() - 1;

    while lo <= hi {
        // Key is in a[lo..hi] or not present
        // If lo == hi, then mid == lo = hi
        let mid = lo + (hi - lo) / 2;

        match Ord::cmp(&searched, &array[mid]) {
            Less => {
                hi = mid - 1;
            }
            Equal => {
                return mid as i32;
            }
            Greater => {
                lo = mid + 1;
            }
        }
    }

    // Key not found, li and hi did cross
    -1
}
