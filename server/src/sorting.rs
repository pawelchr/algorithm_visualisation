use std::time::Instant;
use crate::routes::SortResult;


pub fn selection_sort(mut vec_of_numbers: Vec<i64>) -> SortResult {
    let start = Instant::now();
    let mut array_accesses = 0;
    let mut sort_history = vec_of_numbers.clone();
    for i in 0..vec_of_numbers.len() {
        let mut min_index = i;
        for j in i+1..vec_of_numbers.len() {
            if vec_of_numbers[j] < vec_of_numbers[min_index] {
                min_index = j;
            }
            array_accesses += 1;
        }
        if min_index != i {
            vec_of_numbers.swap(i, min_index);
        }
        sort_history.extend(vec_of_numbers.clone());
    }
    let duration = start.elapsed();
    SortResult {result: sort_history, array_accesses, duration}
}

pub fn bubble_sort(mut vec_of_numbers: Vec<i64>) -> SortResult {
    let start = Instant::now();
    let mut array_accesses = 0;
    let mut sort_history = vec_of_numbers.clone();
    for _ in 0..vec_of_numbers.len() {
        for j in 0..vec_of_numbers.len()-1 {
            if vec_of_numbers[j] > vec_of_numbers[j+1] {
                vec_of_numbers.swap(j, j+1);
                array_accesses += 1;
            }
        }
        sort_history.extend(vec_of_numbers.clone());
    }
    let duration = start.elapsed();
    SortResult {result: sort_history, array_accesses, duration}
}

pub fn insertion_sort(mut vec_of_numbers: Vec<i64>) -> SortResult {
    let start = Instant::now();
    let mut array_accesses = 0;
    let mut sort_history = vec_of_numbers.clone();
    for i in 1..vec_of_numbers.len() {
        let key = vec_of_numbers[i];
        let mut j = i-1;
        while j > 0 && vec_of_numbers[j] > key {
            vec_of_numbers[j+1] = vec_of_numbers[j];
            j -= 1;
            array_accesses += 1;
        }
        vec_of_numbers[j+1] = key;
        sort_history.extend(vec_of_numbers.clone());
    }
    let duration = start.elapsed();
    SortResult {result: sort_history, array_accesses, duration}
}

pub fn merge_sort(mut vec_of_numbers: Vec<i64>) -> SortResult {
    let start = Instant::now();
    let mut array_accesses = 0;
    let mut sort_history = vec_of_numbers.clone();
    let end = vec_of_numbers.len();
    merge_sort_helper(&mut vec_of_numbers, 0, end, &mut sort_history, &mut array_accesses);
    let duration = start.elapsed();
    SortResult {result: sort_history, array_accesses, duration}
}

fn merge_sort_helper(vec_of_numbers: &mut [i64], start: usize, end: usize, sort_history: &mut Vec<i64>, array_accesses: &mut i64) {
    if end - start > 1 {
        let mid = start + (end - start) / 2;
        merge_sort_helper(vec_of_numbers, start, mid, sort_history, array_accesses);
        merge_sort_helper(vec_of_numbers, mid, end, sort_history, array_accesses);
        merge(vec_of_numbers, start, mid, end, sort_history, array_accesses);
    }
}

fn merge(vec_of_numbers: &mut [i64], start: usize, mid: usize, end: usize, sort_history: &mut Vec<i64>, array_accesses: &mut i64) {
    let mut left = vec_of_numbers[start..mid].to_vec();
    let mut right = vec_of_numbers[mid..end].to_vec();
    left.push(i64::MAX);
    right.push(i64::MAX);
    let (mut i, mut j) = (0, 0);
    for item in &mut vec_of_numbers[start..end] {
        if left[i] <= right[j] {
            *item = left[i];
            i += 1;
            *array_accesses += 1;
        } else {
            *item = right[j];
            j += 1;
            *array_accesses += 1;
        }
    }
    sort_history.extend(vec_of_numbers.to_vec());
}

pub fn quick_sort(mut vec_of_numbers: Vec<i64>) -> SortResult {
    let start = Instant::now();
    let mut array_accesses = 0;
    let mut sort_history = vec_of_numbers.clone();
    let high = vec_of_numbers.len() - 1;
    quick_sort_helper(&mut vec_of_numbers, 0, high, &mut sort_history, &mut array_accesses);
    let duration = start.elapsed();
    SortResult {result: sort_history, array_accesses, duration}
}

fn quick_sort_helper(vec_of_numbers: &mut Vec<i64>, low: usize, high: usize, sort_history: &mut Vec<i64>, array_accesses: &mut i64) {
    if low < high {
        let pi = partition(vec_of_numbers, low, high, sort_history, array_accesses);
        if pi > 0 {  // prevent overflow
            quick_sort_helper(vec_of_numbers, low, pi - 1, sort_history, array_accesses);
        }
        quick_sort_helper(vec_of_numbers, pi + 1, high, sort_history, array_accesses);
    }
}

fn partition(vec_of_numbers: &mut [i64], low: usize, high: usize, sort_history: &mut Vec<i64>, array_accesses: &mut i64) -> usize {
    let pivot = vec_of_numbers[high];
    let mut i = low;
    for j in low..high {
        if vec_of_numbers[j] < pivot {
            vec_of_numbers.swap(i, j);
            i += 1;
            sort_history.extend(vec_of_numbers.to_owned());
            *array_accesses += 1;
        }
    }
    vec_of_numbers.swap(i, high);
    sort_history.extend(vec_of_numbers.to_owned());
    i
}

pub fn heap_sort(mut vec_of_numbers: Vec<i64>) -> SortResult {
    let start = Instant::now();
    let mut array_accesses = 0;
    let mut sort_history = vec_of_numbers.clone();
    let end = vec_of_numbers.len();
    for i in (0..end / 2).rev() {
        heapify(&mut vec_of_numbers, end, i, &mut sort_history, &mut array_accesses);
    }
    let duration = start.elapsed();
    SortResult {result: sort_history, array_accesses, duration}
}

fn heapify(vec_of_numbers: &mut Vec<i64>, end: usize, i: usize, sort_history: &mut Vec<i64>, array_accesses: &mut i64) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;
    if left < end && vec_of_numbers[left] > vec_of_numbers[largest] {
        largest = left;
    }
    if right < end && vec_of_numbers[right] > vec_of_numbers[largest] {
        largest = right;
    }
    if largest != i {
        vec_of_numbers.swap(i, largest);
        *array_accesses += 1;
        sort_history.extend(vec_of_numbers.clone());
        heapify(vec_of_numbers, end, largest, sort_history, array_accesses);
    }
}
