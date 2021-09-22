use algorithms::*;
use rand::prelude::*;

fn get_data() -> Vec<i32> {
    let mut rng = rand::thread_rng();

    let n: usize = 1_000_000;
    // let n = 10;
    // let n = n.pow(rng.gen_range(3..=6));
    // let n = 1000;

    let mut res = Vec::with_capacity(n);

    for _ in 0..n {
        res.push(rng.gen_range(1..=1_000_000));
    }

    println!("Size of data = {}", n);
    println!("First 10 = {:?}", res[..10].to_vec());

    res
}

#[test]
fn test_exchange_sort() {
    let mut data = get_data();
    let mut sorted_data = data.clone();
    sorted_data.sort();
    exchange_sort(&mut data);
    assert_eq!(data, sorted_data);
}

#[test]
fn test_choice_sort() {
    let mut data = get_data();
    let mut sorted_data = data.clone();
    sorted_data.sort();
    choice_sort(&mut data);
    assert_eq!(data, sorted_data);
}

#[test]
fn test_bubble_sort() {
    let mut data = get_data();
    let mut sorted_data = data.clone();
    sorted_data.sort();
    bubble_sort(&mut data);
    assert_eq!(data, sorted_data);
}

#[test]
fn test_merge_sort() {
    let mut data = get_data();
    let mut sorted_data = data.clone();
    sorted_data.sort();
    assert_eq!(merge_sort(&mut data), sorted_data);
}

#[test]
fn test_quick_sort() {
    let mut data = get_data();
    let mut sorted_data = data.clone();
    sorted_data.sort();
    quick_sort(&mut data);
    assert_eq!(data, sorted_data);
}

#[test]
fn test_hoar_sort() {
    let mut data = get_data();
    let mut sorted_data = data.clone();
    sorted_data.sort();
    hoar_sort(&mut data);
    assert_eq!(data, sorted_data);
}

#[test]
fn test_shellsort() {
    let mut data = get_data();
    let mut sorted_data = data.clone();
    sorted_data.sort();
    shellsort(&mut data);
    assert_eq!(data, sorted_data);
}

#[test]
fn test_counting_sort() {
    let mut data: Vec<usize> = get_data().iter().map(|i| *i as usize).collect();
    let mut sorted_data = data.clone();
    sorted_data.sort();
    assert_eq!(counting_sort(&mut data), sorted_data);
}

#[test]
fn test_heapsort() {
    let mut data = get_data();
    let mut sorted_data = data.clone();
    sorted_data.sort();
    assert_eq!(heapsort(&mut data), sorted_data);
}

#[test]
fn test_heap() {
    let data = get_data();
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
    let range = |from: usize, to: usize, step: i32| {
        let mut from = from as i32;
        let to = to as i32;

        std::iter::from_fn(move || {
            if (step < 0 && from <= to) || (step > 0 && from >= to) {
                None
            } else {
                let res = Some(from as usize);
                from += step;
                res
            }
        })
    };

    range(1, 10, 1).for_each(|i| print!("{} ", i));
    println!();
    range(11, 0, -2).for_each(|i| print!("{} ", i));
    println!();
}
