pub mod navbar;
pub mod sorting_chart;

use std::vec;

use leptos::*;
use sorting_chart::SortingChart;

#[component]
pub fn App() -> impl IntoView {
    let (sorting_type, set_sorting_type) = create_signal(String::from(""));
    let (vec_to_sort, set_vec_to_sort) = create_signal(vec![1.0]);
    let (input_value, set_input_value) = create_signal("".to_string());
    let input_element: NodeRef<html::Input> = create_node_ref();
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let value = input_element()
        .expect("<input> should be mounted")
        .value();
    set_input_value(value);
    set_vec_to_sort(create_vec_from_string(input_value()));
    };

    view! {
        <select
            on:change=move |ev| {
                let new_sorting_type = event_target_value(&ev);
                set_sorting_type(new_sorting_type);
            }
            prop:value=move || sorting_type.get()
        >
        <option value="bubble">"Bubble Sort"</option>
        <option value="insert">"Insert Sort"</option>
        <option value="quick">"Quick Sort"</option>
        <option value="merge">"Merge Sort"</option>
        </select>
        <form on:submit=on_submit>
            <input type="text"
                value=input_value
                node_ref=input_element
                />
            <input type="submit" value="Submit"/>
        </form>

        <SortingChart data=vec_to_sort/>

        <p> "input value:"{move || input_value()} "\n" 
    "vec_to_sort: "{move || vec_to_sort()}</p>
    }
}


fn create_vec_from_string(value: String) -> Vec<f64> {
    value.split(|c: char| c == ' ' || c == ',')
    .filter_map(|s| {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return None
    }
    else {
        s.parse::<f64>().ok()
        }
    })
    .collect::<Vec<_>>()
}