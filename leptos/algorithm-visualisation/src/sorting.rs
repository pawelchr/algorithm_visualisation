use console_log::init;
use leptos_charts::Color;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, strum_macros::Display)]
pub enum SortType {
    Bubble,
    Insert,
    Quick,
    Merge,
    Invalid,
}

pub struct SortingResult<'a> {
    steps: Steps<'a>,
    time: Duration,
}

impl<'a> SortingResult<'a> {
    fn new(steps: Steps<'a>, time: Duration) -> Self {
        Self { steps, time }
    }
}

struct Steps<'a> {
    steps: Vec<Vec<f64>>,
    palette: Vec<Vec<Color<'a>>>,
}

impl<'a> Steps<'a> {
    fn new() -> Self {
        Self {
            steps: vec![],
            palette: vec![],
        }
    }

    fn push(&mut self, step: Vec<f64>, palette: Vec<Color<'a>>) {
        self.steps.push(step);
        self.palette.push(palette);
    }
}

pub fn bubble_sort<'a>(arr: &'a mut Vec<f64>) -> SortingResult {
    let mut swapped = true;
    let start = Instant::now();
    let base_color: Vec<Color<'a>> = vec![Color::RGB(108, 108, 108); arr.len() - 1];
    let mut steps: Steps<'a> = Steps::<'a>::new();

    while swapped {
        swapped = false;
        steps.push(arr.clone(), base_color.clone());
        for i in 0..arr.len() - 1 {
            let mut palette = base_color.clone();
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
            palette[i] = Color::RGB(0, 255, 0);
            palette[i + 1] = Color::RGB(0, 255, 0);
            steps.push(arr.clone(), palette)
        }
    }
    let duration = start.elapsed();

    SortingResult::<'a>::new(steps, duration)
}
