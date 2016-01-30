#![feature(test)]
extern crate sorter;
extern crate test;
use test::Bencher;
use sorter::*;

fn make_test_vec() -> Vec<Vec<u8>> {
    let mut make_test_vec = vec![10, 5, 1, 2, 4];
    let mut main_vec = vec![make_test_vec];

    make_test_vec = vec![1, 2, 3, 4, 5, 6, 7];
    main_vec.push(make_test_vec);
    make_test_vec = vec![1, 2, 3, 4, 5, 6];
    main_vec.push(make_test_vec);
    make_test_vec = vec![7, 6, 5, 4, 3, 2, 1];
    main_vec.push(make_test_vec);
    make_test_vec = vec![5, 10, 1, 3, 5, 2, 3];
    main_vec.push(make_test_vec);
    make_test_vec = include_bytes!("../test_string.txt").to_vec();
    main_vec.push(make_test_vec);

    main_vec
}

#[bench]
fn bubble_sort_bench(b: &mut Bencher) {
    let mut main_vec = make_test_vec();

    b.iter(|| { for array in main_vec.iter_mut() { bubble_sort(array); } } );
}

#[bench]
fn insertion_sort_bench(b: &mut Bencher) {
    let mut main_vec = make_test_vec();

    b.iter(|| { for array in main_vec.iter_mut() { insertion_sort(array); } } );
}

#[bench]
fn insertion_sort_binary_bench(b: &mut Bencher) {
    let mut main_vec = make_test_vec();

    b.iter(|| { for array in main_vec.iter_mut() { binary_insertion_sort(array); } } );
}

#[bench]
fn quick_sort_bench(b: &mut Bencher) {
    let mut main_vec = make_test_vec();
    b.iter(|| { for array in main_vec.iter_mut() { quick_sort(array); } } );
}

#[bench]
fn heap_sort_bench(b: &mut Bencher) {
    let mut main_vec = make_test_vec();

    b.iter(|| { for array in main_vec.iter_mut() { heap_sort(array); } } );
}

#[bench]
fn shaker_sort_bench(b: &mut Bencher) {
    let mut main_vec = make_test_vec();

    b.iter(|| { for array in main_vec.iter_mut() { shaker_sort(array); } } );
}

#[bench]
fn shell_sort_bench(b: &mut Bencher) {
    let mut main_vec = make_test_vec();

    b.iter(|| { for array in main_vec.iter_mut() { shell_sort(array); } } );
}

