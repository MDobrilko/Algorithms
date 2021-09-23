use rand::seq::SliceRandom;
use std::cmp::Ordering;

pub fn bubble_sort<T: Ord>(data: &mut Vec<T>) {
    for i in 1..data.len() {
        let mut j = i;
        while j > 0 && data[j] < data[j - 1] {
            data.swap(j, j - 1);
            j -= 1;
        }
    }
}

pub fn choice_sort<T: Ord>(data: &mut Vec<T>) {
    for i in 0..(data.len() - 1) {
        let min_idx = data
            .iter()
            .enumerate()
            .skip(i)
            .fold(
                i,
                |min_idx, (idx, elem)| if data[min_idx] > *elem { idx } else { min_idx },
            );

        data.swap(min_idx, i);
    }
}

pub fn exchange_sort<T: Ord>(data: &mut Vec<T>) {
    for i in 0..(data.len() - 1) {
        for j in (i + 1)..data.len() {
            if data[i] > data[j] {
                data.swap(i, j);
            }
        }
    }
}

pub fn merge_sort<T: Ord + Copy>(data: &[T]) -> Vec<T> {
    let mut sorted = Vec::with_capacity(data.len());

    let left_idx = 0;
    let right_idx = data.len();
    let mid_idx = (left_idx + right_idx) / 2;

    if right_idx - left_idx <= 1 {
        sorted.extend_from_slice(data);
        return sorted;
    }

    let sorted_left_side = merge_sort(&data[left_idx..mid_idx]);
    let sorted_right_size = merge_sort(&data[mid_idx..right_idx]);

    let mut left_iter = sorted_left_side.iter().peekable();
    let mut right_iter = sorted_right_size.iter().peekable();
    loop {
        sorted.push(match (left_iter.peek(), right_iter.peek()) {
            (Some(&&left_num), Some(&&right_num)) => {
                if left_num <= right_num {
                    *left_iter.next().unwrap()
                } else {
                    *right_iter.next().unwrap()
                }
            }
            (Some(_), None) => *left_iter.next().unwrap(),
            (None, Some(_)) => *right_iter.next().unwrap(),
            (None, None) => break,
        });
    }

    sorted
}

pub fn quick_sort<T: Ord + Copy>(data: &mut [T]) {
    let size = data.len();
    if size <= 2 {
        if size == 2 && data[0] > data[1] {
            data.swap(0, 1);
        }
        return;
    }

    let mid_elem = *data.choose(&mut rand::thread_rng()).unwrap();
    let mut left_idx = 0;
    let mut right_idx = size - 1;

    while left_idx < right_idx {
        while data[left_idx] < mid_elem {
            left_idx += 1;
        }
        while data[right_idx] > mid_elem {
            right_idx -= 1;
        }

        if left_idx < right_idx {
            data.swap(left_idx, right_idx);
            left_idx += 1;
            right_idx -= 1;
        }
    }

    if right_idx > 0 {
        quick_sort(&mut data[..=right_idx]);
    }
    if left_idx < data.len() {
        quick_sort(&mut data[left_idx..]);
    }
}

pub fn hoar_sort<T: Ord + Copy>(data: &mut [T]) {
    let size = data.len();
    if data.is_empty() {
        return;
    }

    let element_separator = *data.choose(&mut rand::thread_rng()).unwrap();

    let mut smaller_elements_border = 0;
    let mut current_idx = 0;
    let mut larger_elements_border = size - 1;

    while current_idx <= larger_elements_border {
        match data[current_idx].cmp(&element_separator) {
            Ordering::Less => {
                data.swap(smaller_elements_border, current_idx);
                smaller_elements_border += 1;
                current_idx += 1;
            }
            Ordering::Equal => current_idx += 1,
            Ordering::Greater => {
                data.swap(current_idx, larger_elements_border);
                larger_elements_border -= 1;
            }
        }
    }

    hoar_sort(&mut data[..smaller_elements_border]);
    hoar_sort(&mut data[larger_elements_border + 1..]);
}

pub fn shellsort<T: Ord + Copy>(data: &mut [T]) {
    let size = data.len();
    let mut series_size = 1;
    while series_size < size / 10 {
        series_size = 3 * series_size + 1;
    }

    let range = |from: usize, to: usize, step: i32| {
        let mut from = from as i32;
        let to = to as i32;

        std::iter::from_fn(move || {
            if (step < 0 && from < to) || (step > 0 && from > to) {
                None
            } else {
                let res = Some(from as usize);
                from += step;
                res
            }
        })
    };

    let mut step = series_size as i32;
    while series_size > 0 {
        for offset in 0..series_size {
            for current_idx in range(offset + series_size, size - 1, step) {
                range(current_idx - series_size, 0, -step)
                    .zip(range(current_idx, 0, -step))
                    .find_map(|(prev_idx, idx)| {
                        if data[prev_idx] > data[idx] {
                            data.swap(prev_idx, idx);
                            None
                        } else {
                            Some(())
                        }
                    });
            }
        }
        series_size /= 3;
        step = series_size as i32;
    }
}

pub fn counting_sort(data: &[usize]) -> Vec<usize> {
    let mut sorted = Vec::with_capacity(data.len());

    let mut counts = Vec::new();

    for &elem in data {
        if elem >= counts.len() {
            counts.resize(elem + 1, 0usize);
        }
        counts[elem] += 1;
    }

    for (elem, &count) in counts.iter().enumerate() {
        for _ in 0..count {
            sorted.push(elem);
        }
    }

    sorted
}

pub fn heapsort<T: Ord + Copy>(data: &mut [T]) -> Vec<T> {
    let mut heap = crate::Heap::new();

    data.iter().for_each(|&elem| heap.push(elem));

    let mut sorted: Vec<T> = Vec::with_capacity(data.len());
    while let Some(elem) = heap.pop() {
        sorted.push(elem);
    }

    sorted
}
