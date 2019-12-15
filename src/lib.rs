pub mod intcode;
pub mod permute;

use std::cmp::Ordering;

pub fn ordering_to_int(ord: Ordering) -> isize {
    match ord {
        Ordering::Less => -1,
        Ordering::Greater => 1,
        Ordering::Equal => 0,
    }
}
