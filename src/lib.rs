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

/// ShiftDown for heap sort
fn shift_down<T: Ord>(victim: &mut [T], start: usize, ending: usize) {
    let mut root = start;
    loop {
        let mut child = root * 2 + 1;

        if child > ending { break; }
        else if child != ending && victim[child] < victim[child+1] { child += 1; }

        if victim[root] < victim[child] {
            victim.swap(root, child);
            root = child;
        }
        else { break; }
    }
}

/// Implementation of heap sort
pub fn heap_sort<T: Ord>(victim: &mut [T]) {
    let mut last = victim.len() - 1;
    //remember that range is exclusive for right element
    for i in (0..(last - 1) / 2).rev() { shift_down(victim, i, last); }

    last += 1;
    for i in (1..last).rev() {
        victim.swap(0, i);
        shift_down(victim, 0, i-1); }
}

/// Implementation of quick sort
pub fn quick_sort<T: Ord>(victim: &mut [T]) {
    let mut last: usize = victim.len();
    if last < 2 { return; }

    let middle = last / 2;
    last -= 1;

    //choice of pivot and partition
    victim.swap(middle, last);

    let mut pivot_index = 0;
    for i in 0..last {
        if victim[i] <= victim[last] {
            victim.swap(i, pivot_index);
            pivot_index += 1;
        }
    }

    victim.swap(pivot_index, last);

    //next sort
    quick_sort(&mut victim[0..pivot_index]);
    quick_sort(&mut victim[pivot_index+1..last+1]);
}

/// Implementation of bubble sort
pub fn bubble_sort<T: Ord>(victim: &mut [T]) {
    for _ in 0..victim.len() {
        for j in 1..victim.len() {
            if victim[j-1] > victim[j] { victim.swap(j-1, j); }
        }
    }
}
