/**
 * Accept an array of integers and which are mutable reference
 */
pub fn get_last(array: &mut [i32]) -> i32 {
    array[array.len() - 1]
}

pub fn remove_first(array: &mut [i32]) -> Vec<i32> {
    let mut new_vec = array.to_vec();
    new_vec.remove(0);
    new_vec
}

pub fn print_middle_item(array: &mut [i32]) {
    // if even then print /2 and /2+1
    // if odd then n-1/2
    if is_even(array) {
        let middle = array.len() / 2;
        println!("middles are: {} {}", array[middle - 1], array[middle]);
    } else {
        let middle = (array.len() - 1) / 2;
        print!("middle is: {}", array[middle]);
    }
}

pub fn is_even(arr: &mut [i32]) -> bool {
    arr.len() % 2 == 0
}
