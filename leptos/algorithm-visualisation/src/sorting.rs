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

pub fn insertion_sort(mut arr: Vec<f64>) -> SortingResult {
    let mut steps: Steps = Steps::new();
    let base_color = vec![BarColor::Grey; arr.len()];
    let end_color = vec![BarColor::Orange; arr.len()];
    steps.push(arr.clone(), base_color.clone());

    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            let mut palette = base_color.clone();
            palette[j - 1] = BarColor::Green;
            palette[j] = BarColor::Green;
            steps.push(arr.clone(), palette.clone());

            arr.swap(j - 1, j);
            steps.push(arr.clone(), palette);
            j -= 1;
        }
    }

    steps.push(arr.clone(), end_color);
    SortingResult::new(steps)
}

pub fn quick_sort(arr: Vec<f64>) -> SortingResult {
    let mut steps: Steps = Steps::new();
    let base_color = vec![BarColor::Grey; arr.len()];
    steps.push(arr.clone(), base_color.clone());

    fn quick_sort_recursive(arr: &mut Vec<f64>, low: usize, high: usize, steps: &mut Steps, base_color: &Vec<BarColor>) {
        if low < high {
            let p = partition(arr, low, high, steps, base_color);
            if p > 0 {
                quick_sort_recursive(arr, low, p - 1, steps, base_color);
            }
            quick_sort_recursive(arr, p + 1, high, steps, base_color);
        }
    }

    fn partition(arr: &mut Vec<f64>, low: usize, high: usize, steps: &mut Steps, base_color: &Vec<BarColor>) -> usize {
        let pivot = arr[high];
        let mut i = low;

        let mut palette = base_color.clone();
        palette[high] = BarColor::Green;
        steps.push(arr.clone(), palette.clone());

        for j in low..high {
            palette = base_color.clone();
            palette[high] = BarColor::Green;
            palette[j] = BarColor::Green; 
            if i > low {
                palette[i - 1] = BarColor::Green;
            }
            steps.push(arr.clone(), palette.clone());

            if arr[j] <= pivot {
                arr.swap(i, j);
                palette[i] = BarColor::Green;
                steps.push(arr.clone(), palette.clone());
                i += 1;
            }
        }

        arr.swap(i, high);
        palette = base_color.clone();
        palette[i] = BarColor::Orange; 
        for k in low..i {
            palette[k] = BarColor::Orange;
        }
        steps.push(arr.clone(), palette);

        i
    }

    let mut arr_clone = arr.clone();
    if !arr_clone.is_empty() {
        let arr_length = arr_clone.len() - 1;
        quick_sort_recursive(&mut arr_clone, 0, arr_length, &mut steps, &base_color);
    }
    
    let final_palette = vec![BarColor::Orange; arr_clone.len()];
    steps.push(arr_clone.clone(), final_palette);

    SortingResult::new(steps)
}


// pub fn bubble_sort_test(mut arr: Vec<f64>) -> SortRes {
//     let mut swapped = true;
//     let base_color = vec![Colour::from_rgb(192, 192, 192); arr.len()];
//     let end_color = vec![Colour::from_rgb(255, 127, 80); arr.len()];
//     let mut steps_test: StepsTest = StepsTest::new();

//     steps_test.push(create_step(&arr, &base_color));

//     while swapped {
//         swapped = false;
//         for i in 0..arr.len() - 1 {
//             let mut palette = base_color.clone();
//             palette[i] = Colour::from_rgb(50, 205, 50);
//             palette[i + 1] = Colour::from_rgb(50, 205, 50);
//             steps_test.push(create_step(&arr, &palette));

//             if arr[i] > arr[i + 1] {
//                 arr.swap(i, i + 1);
//                 swapped = true;
//                 palette[i] = Colour::from_rgb(50, 205, 50);
//                 palette[i + 1] = Colour::from_rgb(50, 205, 50);
//                 steps_test.push(create_step(&arr, &palette));
//             }
//         }
//     }
//     steps_test.push(create_step(&arr, &end_color));

//     SortRes::new(steps_test)
// }

// fn create_step(arr: &Vec<f64>, colors: &Vec<BarColor>) -> Vec<Steps> {
//     arr.iter()
//         .enumerate()
//         .map(|(index, &y1)| Steps {
//             x: (index + 1) as f64,
//             y1,
//             colour: colors[index].clone(),
//         })
//         .collect()
// }

