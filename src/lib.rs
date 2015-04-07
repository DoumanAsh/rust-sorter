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

/// Implementation of shaker sort
pub fn shaker_sort<T: Ord>(victim: &mut [T]) {
    for _ in 0..victim.len() / 2 + 1 {
        let mut begin: usize = 0;
        let mut ending: usize = victim.len() - 1;

        while begin <= ending {
            if victim[begin] > victim[begin+1] { victim.swap(begin, begin+1); }
            if victim[ending-1] > victim[ending] { victim.swap(ending, ending-1); }
            begin += 1;
            ending -= 1;
        }
    }
}

/// Implementation of quick sort
/// Unstable?
fn int_quick_sort<T: Ord + Clone>(victim: &mut [T], left: usize, right: usize) {
    if left >= right { return; }

    let middle = victim[(left+right)/2].clone();
    let mut left_temp = left;
    let mut right_temp = right;

    while left_temp <= right_temp {
        while left_temp <= right && victim[left_temp] < middle { left_temp += 1; }
        while right_temp >= left && victim[right_temp] > middle { right_temp -= 1; }
        if left_temp <= right_temp {
            victim.swap(left_temp, right_temp);
            left_temp += 1;
            right_temp -= 1;
        }
    }

    int_quick_sort(victim, left, right_temp);
    int_quick_sort(victim, left_temp, right);
}

/// Quick sort wrapper around implementation
pub fn quick_sort<T: Ord + Clone>(victim: &mut [T]) {
    let last: usize = victim.len() - 1;
    int_quick_sort(victim, 0, last);
}

/// Implementation of bubble sort
pub fn bubble_sort<T: Ord>(victim: &mut [T]) {
    for _ in 0..victim.len() {
        for j in 1..victim.len() {
            if victim[j-1] > victim[j] { victim.swap(j-1, j); }
        }
    }
}
