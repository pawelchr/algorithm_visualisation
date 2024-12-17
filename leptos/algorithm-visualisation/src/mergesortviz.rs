use leptos::*;
use rand::Rng;
use std::time::Duration;
use std::pin::Pin;
use std::future::Future;
use crate::navbar::NavBar;
use crate::sorting_controls::SortingControls;
use crate::sorting_info::{AlgorithmInfoPanel, MERGE_SORT_INFO};

#[derive(Clone)]
struct ArrayElement {
    value: i32,
    in_bottom_view: bool,
}

#[component]
pub fn MergeSortVisualizer() -> impl IntoView {
    let (array_elements, set_array_elements) = create_signal(vec![]);
    let (sorting, set_sorting) = create_signal(false);
    let (comparing_indices, set_comparing_indices) = create_signal(Vec::new());
    let (array_size, set_array_size) = create_signal(15);
    let (sorted_indices, set_sorted_indices) = create_signal(Vec::new());
    
    // Initialize array with random values
    let generate_array = move |size: usize| {
        let mut rng = rand::thread_rng();
        let new_array: Vec<i32> = (0..size)
            .map(|_| rng.gen_range(10..100))
            .collect();
        let new_elements: Vec<ArrayElement> = new_array
            .iter()
            .map(|&value| ArrayElement { value, in_bottom_view: false })
            .collect();
        
        set_array_elements(new_elements);
        set_comparing_indices(Vec::new());
        set_sorted_indices(Vec::new());
    };

    async fn merge_async(
        elements: &mut Vec<ArrayElement>,
        start: usize,
        mid: usize,
        end: usize,
        set_array_elements: WriteSignal<Vec<ArrayElement>>,
        set_comparing_indices: WriteSignal<Vec<usize>>,
        sorting: ReadSignal<bool>,
    ) -> bool {
        let left = elements[start..=mid].to_vec();
        let right = elements[mid + 1..=end].to_vec();
        
        // Move elements being merged to bottom view
        for i in start..=end {
            if !sorting.get() { return false; }
            elements[i].in_bottom_view = true;
        }
        set_array_elements(elements.clone());
        delay(100).await;
        
        let mut i = 0;
        let mut j = 0;
        let mut k = start;
        
        while i < left.len() && j < right.len() {
            if !sorting.get() { return false; }
            
            set_comparing_indices(vec![start + i, mid + 1 + j]);
            delay(100).await;
            
            if left[i].value <= right[j].value {
                elements[k] = left[i].clone();
                elements[k].in_bottom_view = false;
                i += 1;
            } else {
                elements[k] = right[j].clone();
                elements[k].in_bottom_view = false;
                j += 1;
            }
            
            set_array_elements(elements.clone());
            k += 1;
            delay(100).await;
        }
        
        while i < left.len() {
            if !sorting.get() { return false; }
            
            elements[k] = left[i].clone();
            elements[k].in_bottom_view = false;
            set_array_elements(elements.clone());
            i += 1;
            k += 1;
            delay(100).await;
        }
        
        while j < right.len() {
            if !sorting.get() { return false; }
            
            elements[k] = right[j].clone();
            elements[k].in_bottom_view = false;
            set_array_elements(elements.clone());
            j += 1;
            k += 1;
            delay(100).await;
        }
        
        set_comparing_indices(Vec::new());
        true
    }

    fn merge_sort_async<'a>(
        elements: &'a mut Vec<ArrayElement>,
        start: usize,
        end: usize,
        set_array_elements: WriteSignal<Vec<ArrayElement>>,
        set_comparing_indices: WriteSignal<Vec<usize>>,
        sorted_indices: ReadSignal<Vec<usize>>,
        set_sorted_indices: WriteSignal<Vec<usize>>,
        sorting: ReadSignal<bool>,
    ) -> Pin<Box<dyn Future<Output = bool> + 'a>> {
        Box::pin(async move {
            if !sorting.get() { return false; }

            if start < end {
                let mid = (start + end) / 2;
                
                // Sort left half
                let left_completed = merge_sort_async(
                    elements,
                    start,
                    mid,
                    set_array_elements,
                    set_comparing_indices,
                    sorted_indices,
                    set_sorted_indices,
                    sorting,
                ).await;

                if !left_completed { return false; }
                
                // Sort right half
                let right_completed = merge_sort_async(
                    elements,
                    mid + 1,
                    end,
                    set_array_elements,
                    set_comparing_indices,
                    sorted_indices,
                    set_sorted_indices,
                    sorting,
                ).await;

                if !right_completed { return false; }
                
                // Merge the sorted halves
                let merge_completed = merge_async(
                    elements,
                    start,
                    mid,
                    end,
                    set_array_elements,
                    set_comparing_indices,
                    sorting,
                ).await;

                if !merge_completed { return false; }
                
                let mut sorted = sorted_indices.get();
                for idx in start..=end {
                    if !sorted.contains(&idx) {
                        sorted.push(idx);
                    }
                }
                set_sorted_indices(sorted);
            }
            true
        })
    }

    let merge_sort = move |_| {
        set_sorting.set(true);
        set_sorted_indices(Vec::new());
        
        create_local_resource(
            move || (),
            move |_| async move {
                let mut current_elements = array_elements.get();
                let len = current_elements.len();
                
                let completed = merge_sort_async(
                    &mut current_elements,
                    0,
                    len - 1,
                    set_array_elements,
                    set_comparing_indices,
                    sorted_indices,
                    set_sorted_indices,
                    sorting,
                ).await;
                
                // Only update final state if sort completed successfully
                if completed {
                    set_sorted_indices((0..len).collect());
                } else {
                    // Clean up visualization state if cancelled
                    set_comparing_indices(Vec::new());
                    set_sorted_indices(Vec::new());
                    
                    let mut elements = array_elements.get();
                    for element in elements.iter_mut() {
                        element.in_bottom_view = false;
                    }
                    set_array_elements(elements);
                }
                
                set_sorting.set(false);
            },
        );
    };

    let on_size_change = move |ev| {
        let new_size = event_target_value(&ev)
            .parse::<usize>()
            .unwrap_or(50);
        set_array_size(new_size);
        generate_array(new_size);
    };

    let stop_sorting = move |_| {
        set_sorting.set(false);
    };

    create_effect(move |_| {
        generate_array(array_size());
    });

    view! {
        <NavBar/>
        <SortingControls
            array_size=array_size
            is_sorting=sorting
            on_generate=move |_| generate_array(array_size())
            on_sort=merge_sort
            on_size_change=on_size_change
        >
            <button
                class="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 transition-colors"
                on:click=stop_sorting
                disabled=move || !sorting.get()
            >
                "Stop Sorting"
            </button>
        </SortingControls>
            
            <div class="relative h-96">
                // Main array view
                <div class="absolute top-0 left-0 right-0 h-2/3 flex items-end gap-1">
                    {move || {
                        array_elements.get().into_iter().enumerate().map(|(idx, element)| {
                            let height = format!("{}%", element.value);
                            let is_comparing = comparing_indices.get().contains(&idx);
                            let is_sorted = sorted_indices.get().contains(&idx);
                            let is_visible = !element.in_bottom_view;
                            
                            let color = if is_comparing {
                                "#22c55e"
                            } else if is_sorted {
                                "#e3963e"
                            } else {
                                "#6c6c6c"
                            };
                            
                            view! {
                                <div
                                    class="flex-1 transition-all duration-300"
                                    style=move || format!(
                                        "height: {}; background-color: {}; opacity: {}",
                                        height,
                                        color,
                                        if is_visible { "1" } else { "0" }
                                    )
                                >
                                </div>
                            }
                        }).collect_view()
                    }}
                </div>
                
                // Bottom merge view
                <div class="absolute bottom-0 left-0 right-0 h-1/3 flex items-end gap-1 border-t-2 border-gray-300">
                    {move || {
                        array_elements.get().into_iter().enumerate().map(|(idx, element)| {
                            let height = format!("{}%", element.value);
                            let is_comparing = comparing_indices.get().contains(&idx);
                            let is_visible = element.in_bottom_view;
                            
                            let color = if is_comparing {
                                "#22c55e"
                            } else {
                                "#ef4444"
                            };
                            
                            view! {
                                <div
                                    class="flex-1 transition-all duration-300"
                                    style=move || format!(
                                        "height: {}; background-color: {}; opacity: {}",
                                        height,
                                        color,
                                        if is_visible { "1" } else { "0" }
                                    )
                                >
                                </div>
                            }
                        }).collect_view()
                    }}
                </div>
        </div>
        <AlgorithmInfoPanel algorithm_info=MERGE_SORT_INFO/>
    }
}

fn delay(ms: u64) -> impl std::future::Future<Output = ()> {
    async move {
        let (tx, rx) = futures::channel::oneshot::channel::<()>();
        set_timeout(move || {
            let _ = tx.send(());
        }, Duration::from_millis(ms));
        let _ = rx.await;
    }
}
