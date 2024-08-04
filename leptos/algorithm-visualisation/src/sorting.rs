use leptos::{Attribute, IntoAttribute, Oco};
use std::str::FromStr;

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
        // Convert SortType to its string representation
        let value = self.to_string();

        // Create and return an Attribute::String variant
        Attribute::String(Oco::from(value))
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        // Convert boxed SortType to its string representation
        let value = self.to_string();

        // Create and return an Attribute::String variant
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
