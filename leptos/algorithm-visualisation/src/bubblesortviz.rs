use leptos::*;
use rand::Rng;
use std::time::Duration;
use crate::navbar::NavBar;
use crate::sorting_controls::SortingControls;
use crate::sorting_info::{AlgorithmInfoPanel, BUBBLE_SORT_INFO};

#[component]
pub fn BubbleSortVisualizer() -> impl IntoView {
    let (array, set_array) = create_signal(vec![]);
    let (sorting, set_sorting) = create_signal(false);
    let (comparison_idx, set_comparison_idx) = create_signal(None::<(usize, usize)>);
    let (array_size, set_array_size) = create_signal(15);
    let (sorted_indices, set_sorted_indices) = create_signal(Vec::new());
    
    let generate_array = move |size: usize| {
        let mut rng = rand::thread_rng();
        let new_array: Vec<i32> = (0..size)
            .map(|_| rng.gen_range(10..100))
            .collect();
        set_array(new_array);
        set_comparison_idx(None);
        set_sorted_indices(Vec::new());
    };

    let bubble_sort = move |_| {
        set_sorting.set(true);
        set_sorted_indices(Vec::new());
        
        create_local_resource(
            move || (),
            move |_| async move {
                let mut current_array = array.get();
                let len = current_array.len();
                let mut sorted = Vec::new();

                'outer: for i in 0..len {
                    let mut swapped = false;
                    for j in 0..len - i - 1 {
                        // Check if sorting has been stopped
                        if !sorting.get() {
                            set_comparison_idx.set(None);
                            set_sorted_indices(Vec::new());
                            break 'outer;
                        }

                        set_comparison_idx.set(Some((j, j + 1)));
                        
                        if current_array[j] > current_array[j + 1] {
                            current_array.swap(j, j + 1);
                            set_array.set(current_array.clone());
                            swapped = true;
                        }
                        
                        delay(50).await;
                    }
                    
                    if sorting.get() {
                        sorted.push(len - i - 1);
                        set_sorted_indices(sorted.clone());
                        
                        if !swapped {
                            // If no swapping occurred, all remaining elements are sorted
                            for k in 0..len-i-1 {
                                sorted.push(k);
                            }
                            set_sorted_indices(sorted);
                            break;
                        }
                    }
                }
                
                if sorting.get() {
                    set_sorted_indices((0..len).collect());
                }
                
                set_sorting.set(false);
                set_comparison_idx.set(None);
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
            on_sort=bubble_sort
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
        
        <div class="h-96 flex items-end gap-1">
            {move || {
                array.get().into_iter().enumerate().map(|(idx, value)| {
                    let height = format!("{}%", value);
                    let is_comparing = comparison_idx.get()
                        .map(|(i, j)| idx == i || idx == j)
                        .unwrap_or(false);
                    let is_sorted = sorted_indices.get().contains(&idx);
                    
                    let color = if is_comparing {
                        "#22c55e"
                    } else if is_sorted {
                        "#e3963e"
                    } else {
                        "#6c6c6c"
                    };
                    
                    view! {
                        <div
                            class="flex-1"
                            style=move || format!(
                                "height: {}; background-color: {}",
                                height,
                                color
                            )
                        >
                        </div>
                    }
                }).collect_view()
            }}
        </div>
        <AlgorithmInfoPanel algorithm_info=BUBBLE_SORT_INFO/>
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