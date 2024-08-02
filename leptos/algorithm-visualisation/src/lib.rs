pub mod navbar;
pub mod sorting;
pub mod sorting_chart;
use leptos::logging::log;
use std::{str::FromStr, vec};

use leptos::*;
use sorting::{bubble_sort, SortType, SortingResult, Steps};
use sorting_chart::SortingChart;
use std::time::Duration;

#[component]
pub fn App() -> impl IntoView {
    let (sorting_type, set_sorting_type) = create_signal(SortType::Bubble);
    let (sorted_vec, set_sorted_vec) = create_signal(vec![vec![1.0]]);
    let (input_value, set_input_value) = create_signal("".to_string());
    let input_element: NodeRef<html::Input> = create_node_ref();
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let value = input_element().expect("<input> should be mounted").value();
        set_input_value(value);
        let result: SortingResult = SortingResult::new(Steps::new(), Duration::default());
        match sorting_type() {
            SortType::Bubble => result = bubble_sort(&mut create_vec_from_string(input_value())),
            SortType::Quick => result = bubble_sort(&mut create_vec_from_string(input_value())),
            SortType::Insert => result = bubble_sort(&mut create_vec_from_string(input_value())),
            SortType::Merge => result = bubble_sort(&mut create_vec_from_string(input_value())),
        };
        set_sorted_vec(result.steps.steps);
    };

    create_effect(move |_| {
        log!("sorted_vec: {}", sorted_vec());
    });

    view! {
        <select
            on:change=move |ev| {
                set_sorting_type(SortType::from_str(&event_target_value(&ev)).unwrap());
            }
            prop:value=move || sorting_type.get().to_string()
        >
            <option value=SortType::Bubble>"Bubble Sort"</option>
            <option value=SortType::Insert>"Insert Sort"</option>
            <option value=SortType::Quick>"Quick Sort"</option>
            <option value=SortType::Merge>"Merge Sort"</option>
        </select>
        <form on:submit=on_submit>
            <input type="text" value=input_value node_ref=input_element />
            <input type="submit" value="Submit" />
        </form>        <SortingChart data=sorted_vec />

        <p>"input value:"{move || input_value()} "\n" "vec_to_sort: "{move || sorted_vec()}</p>
    }
}

fn create_vec_from_string(value: String) -> Vec<f64> {
    value
        .split(|c: char| c == ' ' || c == ',')
        .filter_map(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                return None;
            } else {
                s.parse::<f64>().ok()
            }
        })
        .collect::<Vec<_>>()
}
