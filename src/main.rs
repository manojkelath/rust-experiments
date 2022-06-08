mod array_utils;
mod tuples_utils;

use array_utils::{get_last, print_middle_item, remove_first};
use tuples_utils::{get_total_score, Score};

fn main() {
    let mut arr = [1, 2, 3, 4, 5, 6];
    println!("last item is : {}", get_last(&mut arr)); //pass a mutable reference
    println!(
        "array after removing first item : {:?}",
        remove_first(&mut arr)
    ); //pass a mutable reference
    print_middle_item(&mut arr);
    let mut scores = [
        Score("A".to_string(), 22),
        Score("B".to_string(), 45),
        Score("C".to_string(), 48),
    ];

    print!("total score is : {}", get_total_score(&mut scores));
}
