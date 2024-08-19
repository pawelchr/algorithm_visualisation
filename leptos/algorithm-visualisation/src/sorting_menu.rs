use std::{str::FromStr, vec};

use leptos::*;
use leptos_router::{Route, RouteProps, Router, RouterProps, Routes, RoutesProps};
use crate::sorting::{bubble_sort, BarColor, SortType};
use crate::sorting_chart::SortingChart;


#[component]
pub fn SortingMenu() -> impl IntoView {
    let (sorting_type, set_sorting_type) = create_signal(SortType::Bubble);
    let (sorted_vec, set_sorted_vec) = create_signal(vec![vec![1.0]]);
    let (palletes, set_palletes) = create_signal(vec![vec![BarColor::Green]]);
    let (input_value, set_input_value) = create_signal("".to_string());
    let input_element: NodeRef<html::Input> = create_node_ref();
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let value = input_element
            .get()
            .expect("<input> should be mounted")
            .value();
        set_input_value(value);
        let read_vector = create_vec_from_string(input_value.get());
        let result = match sorting_type() {
            SortType::Bubble => bubble_sort(read_vector),
            SortType::Quick => bubble_sort(read_vector),
            SortType::Insert => bubble_sort(read_vector),
            SortType::Merge => bubble_sort(read_vector),
        };
        set_palletes(result.steps.palette);
        set_sorted_vec(result.steps.steps);
    };

    view! {
        <div>
            <select
                class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
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
            </form>
            <SortingChart steps=sorted_vec palettes=palletes />

            <p>
                "input value:"{move || input_value.get()} "\n" "vec_to_sort: "
                {move || sorted_vec.get()}
            </p>
        </div>
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