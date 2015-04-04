extern crate sorter;

use sorter::{insertion_sort, shell_sort, bubble_sort};

fn check_order<T: Ord>(array: &mut [T]) -> bool {
    for i in 1..array.len() {
        if array[i-1] > array[i] {
            return false;
        }
    }
    true
}
fn make_test_vec() -> Vec<Vec<i32>> {
    let mut make_test_vec = vec![10, 5, 1, 2, 4];
    let mut main_vec = vec![make_test_vec];

    make_test_vec = vec![1, 2, 3, 4, 5, 6, 7];
    main_vec.push(make_test_vec);
    make_test_vec = vec![7, 6, 5, 4, 3, 2, 1];
    main_vec.push(make_test_vec);
    make_test_vec = vec![5, 10, 1, 3, 5, 2, 3];
    main_vec.push(make_test_vec);

    main_vec
}

#[test]
fn verify_insertion_sort() {
    let mut main_vec = make_test_vec();

    for array in main_vec.iter_mut() {
        insertion_sort(array);
        if !check_order(array) {
            println!("Incorrectly ordered array: {:?}", array);
            assert!(false);
        }
    }
}

#[test]
fn verify_shell_sort() {
    let mut main_vec = make_test_vec();

    for array in main_vec.iter_mut() {
        shell_sort(array);
        if !check_order(array) {
            println!("Incorrectly ordered array: {:?}", array);
            assert!(false);
        }
    }
}

#[test]
fn verify_bubble_sort() {
    let mut main_vec = make_test_vec();

    for array in main_vec.iter_mut() {
        bubble_sort(array);
        if !check_order(array) {
            println!("Incorrectly ordered array: {:?}", array);
            assert!(false);
        }
    }
}