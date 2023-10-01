use algorithms::{
	sortings::*,
	trees::*,
};
use rand::prelude::*;

fn get_data(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();

    let mut res = Vec::with_capacity(n);

    for _ in 0..n {
        res.push(rng.gen_range(1..=1_000_000));
    }

    res
}

#[test]
fn test_exchange_sort() {
    let mut data = get_data(1000);
    let mut sorted_data = data.clone();
    sorted_data.sort();
    exchange_sort(&mut data);
    assert_eq!(data, sorted_data);
}

#[test]
fn test_choice_sort() {
    let mut data = get_data(1000);
    let mut sorted_data = data.clone();
    sorted_data.sort();
    choice_sort(&mut data);
    assert_eq!(data, sorted_data);
}

#[test]
fn test_bubble_sort() {
    let mut data = get_data(1000);
    let mut sorted_data = data.clone();
    sorted_data.sort();
    bubble_sort(&mut data);
    assert_eq!(data, sorted_data);
}

#[test]
fn test_merge_sort() {
    let mut data = get_data(1_000_000);
    let mut sorted_data = data.clone();
    sorted_data.sort();
    assert_eq!(merge_sort(&mut data).into_vec(), sorted_data);
}

#[test]
fn test_quick_sort() {
    let mut data = get_data(1_000_000);
    let mut sorted_data = data.clone();
    sorted_data.sort();
    quick_sort(&mut data);
    assert_eq!(data, sorted_data);
}

#[test]
fn test_hoar_sort() {
    let mut data = get_data(1_000_000);
    let mut sorted_data = data.clone();
    sorted_data.sort();
    hoar_sort(&mut data);
    assert_eq!(data, sorted_data);
}

#[test]
fn test_shellsort() {
    let mut data = get_data(1_000_000);
    let mut sorted_data = data.clone();
    sorted_data.sort();
    shellsort(&mut data);
    assert_eq!(data, sorted_data);
}

#[test]
fn test_counting_sort() {
    let mut data: Vec<usize> = get_data(1_000_000).iter().map(|i| *i as usize).collect();
    let mut sorted_data = data.clone();
    sorted_data.sort();
    assert_eq!(counting_sort(&mut data), sorted_data);
}

#[test]
fn test_heapsort() {
    let mut data = get_data(1_000_000);
    let mut sorted_data = data.clone();
    sorted_data.sort();
    assert_eq!(heapsort(&mut data), sorted_data);
}

#[test]
fn test_heap() {
    let data = get_data(1_000_000);
    let mut sorted_data = data.clone();
    sorted_data.sort();

    let mut heap = Heap::new();
    data.iter().for_each(|elem| heap.push(elem));

    assert_eq!(
        std::iter::from_fn(|| heap.pop().map(|elem| *elem)).collect::<Vec<i32>>(),
        sorted_data
    );
}

#[test]
fn test_range() {
	use algorithms::range;

    let res: Vec<usize> = range(1, 10, 1).collect();
	let correct: Vec<usize> = (1..10 + 1).collect();
	assert_eq!(res, correct);
}
