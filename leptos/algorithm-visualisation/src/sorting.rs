use leptos::{Attribute, IntoAttribute, IntoView, Oco};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct MyData {
    pub x: f64,
    pub y1: f64,
    pub colour: BarColor,
}

impl MyData {
    const fn new(x: f64, y1: f64, colour: BarColor) -> Self {
        Self { x, y1, colour }
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct SortRes {
    pub steps: StepsTest,
}

impl SortRes {
    pub fn new(steps: StepsTest) -> Self {
        SortRes { steps }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct StepsTest{ 
    pub data: Vec<Vec<MyData>>
}

impl StepsTest {
    pub fn new() -> Self {
        StepsTest { data: Vec::new() }
    }

    pub fn push(&mut self, step: Vec<MyData>) {
        self.data.push(step);
    }

    pub fn return_index(self, index: usize) -> Vec<MyData> {
        let temp = self.data.get(index).clone();
        match temp {
            Some(temp) => temp.clone(),
            None => vec![MyData::new(1.0, 1.0, BarColor::Green)],
        }
    }
}

#[derive(Debug, Clone, strum_macros::Display)]
pub enum SortType {
    Bubble,
    Insert,
    Quick,
    Merge,
}

#[derive(Debug, Clone, Copy, PartialEq, strum_macros::Display)]
pub enum BarColor {
    Green,
    Grey,
    Orange,
}

impl FromStr for SortType {
    type Err = ();

    fn from_str(s: &str) -> Result<SortType, Self::Err> {
        match s {
            "Bubble" => Ok(SortType::Bubble),
            "Insert" => Ok(SortType::Insert),
            "Quick" => Ok(SortType::Quick),
            "Merge" => Ok(SortType::Merge),
            _ => Err(()),
        }
    }
}

impl IntoAttribute for SortType {
    fn into_attribute(self) -> Attribute {
        let value = self.to_string();
        Attribute::String(Oco::from(value))
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        let value = self.to_string();
        Attribute::String(Oco::from(value))
    }
}

pub struct SortingResult {
    pub steps: Steps,
}

impl SortingResult {
    pub fn new(steps: Steps) -> Self {
        Self { steps }
    }
}

pub struct Steps {
    pub steps: Vec<Vec<f64>>,
    pub palette: Vec<Vec<BarColor>>,
}

impl Steps {
    pub fn new() -> Self {
        Self {
            steps: vec![],
            palette: vec![],
        }
    }

    fn push(&mut self, step: Vec<f64>, palette: Vec<BarColor>) {
        self.steps.push(step);
        self.palette.push(palette);
    }
}

pub fn bubble_sort(mut arr: Vec<f64>) -> SortingResult {
    let mut swapped = true;
    let base_color = vec![BarColor::Grey; arr.len()];
    let end_color = vec![BarColor::Orange; arr.len()];
    let mut steps: Steps = Steps::new();

    steps.push(arr.clone(), base_color.clone());
    while swapped {
        swapped = false;
        for i in 0..arr.len() - 1 {
            let mut palette = base_color.clone();
            palette[i] = BarColor::Green;
            palette[i + 1] = BarColor::Green;
            steps.push(arr.clone(), palette.clone());
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
                palette[i] = BarColor::Green;
                palette[i + 1] = BarColor::Green;
                steps.push(arr.clone(), palette);
            }
        }
    }
    steps.push(arr.clone(), end_color.clone());

    SortingResult::new(steps)
}

pub fn bubble_sort_test(mut arr: Vec<f64>) -> SortRes {
    let mut swapped = true;
    let base_color = vec![BarColor::Grey; arr.len()];
    let end_color = vec![BarColor::Orange; arr.len()];
    let mut steps_test: StepsTest = StepsTest::new();

    steps_test.push(create_step(&arr, &base_color));

    while swapped {
        swapped = false;
        for i in 0..arr.len() - 1 {
            let mut palette = base_color.clone();
            palette[i] = BarColor::Green;
            palette[i + 1] = BarColor::Green;
            steps_test.push(create_step(&arr, &palette));

            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
                palette[i] = BarColor::Green;
                palette[i + 1] = BarColor::Green;
                steps_test.push(create_step(&arr, &palette));
            }
        }
    }
    steps_test.push(create_step(&arr, &end_color));

    SortRes::new(steps_test)
}

fn create_step(arr: &Vec<f64>, colors: &Vec<BarColor>) -> Vec<MyData> {
    arr.iter()
        .enumerate()
        .map(|(index, &y1)| MyData {
            x: (index + 1) as f64,
            y1,
            colour: colors[index].clone(),
        })
        .collect()
}

