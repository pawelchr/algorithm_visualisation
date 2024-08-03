use leptos::{Attribute, IntoAttribute, Oco};
use leptos_charts::Color;
use std::str::FromStr;

#[derive(Debug, Clone, strum_macros::Display)]
pub enum SortType {
    Bubble,
    Insert,
    Quick,
    Merge,
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

pub struct SortingResult<'a> {
    pub steps: Steps<'a>,
}

impl<'a> SortingResult<'a> {
    pub fn new(steps: Steps<'a>) -> Self {
        Self { steps }
    }
}

pub struct Steps<'a> {
    pub steps: Vec<Vec<f64>>,
    pub palette: Vec<Vec<Color<'a>>>,
}

impl<'a> Steps<'a> {
    pub fn new() -> Self {
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
    let base_color: Vec<Color<'a>> = vec![Color::RGB(108, 108, 108); arr.len()];
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

    SortingResult::<'a>::new(steps)
}
