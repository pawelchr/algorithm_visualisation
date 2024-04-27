use std::time::Instant;

pub fn selection_sort(mut vec_of_numbers: Vec<f64>) -> (Vec<f64>, std::time::Duration) {
    let start = Instant::now();
    let mut sort_history = vec_of_numbers.clone();
    for i in 0..vec_of_numbers.len() {
        let mut min_index = i;
        for j in i+1..vec_of_numbers.len() {
            if vec_of_numbers[j] < vec_of_numbers[min_index] {
                min_index = j;
            }
        }
        if min_index != i {
            vec_of_numbers.swap(i, min_index);
        }
        sort_history.extend(vec_of_numbers.clone());
    }
    let duration = start.elapsed();
    (sort_history, duration)
}

pub fn bubble_sort(mut vec_of_numbers: Vec<f64>) -> (Vec<f64>, std::time::Duration) {
    let start = Instant::now();
    let mut sort_history = vec_of_numbers.clone();
    for _ in 0..vec_of_numbers.len() {
        for j in 0..vec_of_numbers.len()-1 {
            if vec_of_numbers[j] > vec_of_numbers[j+1] {
                vec_of_numbers.swap(j, j+1);
            }
        }
        sort_history.extend(vec_of_numbers.clone());
    }
    let duration = start.elapsed();
    (sort_history, duration)
}

pub fn insertion_sort(mut vec_of_numbers: Vec<f64>) -> (Vec<f64>, std::time::Duration) {
    let start = Instant::now();
    let mut sort_history = vec_of_numbers.clone();
    for i in 1..vec_of_numbers.len() {
        let key = vec_of_numbers[i];
        let mut j = i-1;
        while j > 0 && vec_of_numbers[j] > key {
            vec_of_numbers[j+1] = vec_of_numbers[j];
            j -= 1;
        }
        vec_of_numbers[j+1] = key;
        sort_history.extend(vec_of_numbers.clone());
    }
    let duration = start.elapsed();
    (sort_history, duration)
}

pub fn merge_sort(mut vec_of_numbers: Vec<f64>) -> (Vec<f64>, std::time::Duration) {
    let start = Instant::now();
    let mut sort_history = vec_of_numbers.clone();
    let end = vec_of_numbers.len();
    merge_sort_helper(&mut vec_of_numbers, 0, end, &mut sort_history);
    let duration = start.elapsed();
    (sort_history, duration)
}

fn merge_sort_helper(vec_of_numbers: &mut [f64], start: usize, end: usize, sort_history: &mut Vec<f64>) {
    if end - start > 1 {
        let mid = start + (end - start) / 2;
        merge_sort_helper(vec_of_numbers, start, mid, sort_history);
        merge_sort_helper(vec_of_numbers, mid, end, sort_history);
        merge(vec_of_numbers, start, mid, end, sort_history);
    }
}

fn merge(vec_of_numbers: &mut [f64], start: usize, mid: usize, end: usize, sort_history: &mut Vec<f64>) {
    let mut left = vec_of_numbers[start..mid].to_vec();
    let mut right = vec_of_numbers[mid..end].to_vec();
    left.push(std::f64::INFINITY);
    right.push(std::f64::INFINITY);
    let (mut i, mut j) = (0, 0);
    for item in &mut vec_of_numbers[start..end] {
        if left[i] <= right[j] {
            *item = left[i];
            i += 1;
        } else {
            *item = right[j];
            j += 1;
        }
    }
    sort_history.extend(vec_of_numbers.to_vec());
}

pub fn quick_sort(mut vec_of_numbers: Vec<f64>) -> (Vec<f64>, std::time::Duration) {
    let start = Instant::now();
    let duration = start.elapsed();
    (vec_of_numbers, duration)
}

pub fn heap_sort(mut vec_of_numbers: Vec<f64>) -> (Vec<f64>, std::time::Duration) {
    let start = Instant::now();
    let duration = start.elapsed();
    (vec_of_numbers, duration)
}

pub fn counting_sort(mut vec_of_numbers: Vec<f64>) -> (Vec<f64>, std::time::Duration) {
    let start = Instant::now();
    let duration = start.elapsed();
    (vec_of_numbers, duration)
}