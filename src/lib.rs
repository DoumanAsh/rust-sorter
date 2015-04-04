/// Sorter functions


/// Implementation of insertion sort
/// @victim Mutable array
pub fn insertion_sort<T: Ord>(victim: &mut [T]) {
    for i in 1..victim.len() {
        for j in (0..i).rev() {
            if victim[j+1] < victim[j] { victim.swap(j+1, j); }
            else { break; }
        }
    }
}

/// Implementation of shell sort
pub fn shell_sort<T: Ord + Clone>(victim: &mut [T]) {
    let gaps = [701, 301, 132, 57, 23, 10, 4, 1];

    for gap in gaps.iter() {
        for i in *gap..victim.len() {
            let mut j = i;
            let temp = victim[i].clone();
            while j >= *gap && victim[j-*gap] > temp {
                victim.swap(j, j-*gap);
                j -= *gap;
            }
            victim[j] = temp;
        }
    }
}

/// Implementation of bubble sort
pub fn bubble_sort<T: Ord>(victim: &mut [T]) {
    for _ in 0..victim.len() {
        for j in 1..victim.len() {
            if victim[j-1] > victim[j] { victim.swap(j-1, j); }
        }
    }
}
