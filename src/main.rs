mod array_utils;

use array_utils::{get_last, print_middle_item, remove_first};

fn main() {
    let mut arr = [1, 2, 3, 4, 5, 6];
    println!("last item is - {}", get_last(&mut arr)); //pass a mutable reference
    println!(
        "array after removing first item - {:?}",
        remove_first(&mut arr)
    ); //pass a mutable reference
    print_middle_item(&mut arr);
}
