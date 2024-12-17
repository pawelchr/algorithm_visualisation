use leptos::*;
use rand::Rng;
use std::time::Duration;
use crate::navbar::NavBar;
use crate::sorting_controls::SortingControls;
use crate::sorting_info::{AlgorithmInfoPanel, INSERTION_SORT_INFO};

#[component]
pub fn InsertionSortVisualizer() -> impl IntoView {
    let (array, set_array) = create_signal(vec![]);
    let (sorting, set_sorting) = create_signal(false);
    let (current_idx, set_current_idx) = create_signal(None::<usize>);
    let (comparing_idx, set_comparing_idx) = create_signal(None::<usize>);
    let (array_size, set_array_size) = create_signal(15);
    let (sorted_until, set_sorted_until) = create_signal(None::<usize>);
    
    let generate_array = move |size: usize| {
        let mut rng = rand::thread_rng();
        let new_array: Vec<i32> = (0..size)
            .map(|_| rng.gen_range(10..100))
            .collect();
        set_array(new_array);
        set_current_idx(None);
        set_comparing_idx(None);
        set_sorted_until(None);
    };

    let insertion_sort = move |_: web_sys::MouseEvent| {
        set_sorting.set(true);
        set_sorted_until(Some(0));
        
        create_local_resource(
            move || (),
            move |_| async move {
                let mut current_array = array.get();
                let len = current_array.len();

                for i in 1..len {
                    if !sorting.get() {
                        set_current_idx(None);
                        set_comparing_idx(None);
                        set_sorted_until(None);
                        break;
                    }

                    set_current_idx(Some(i));
                    let mut j = i;
                    
                    while j > 0 {
                        if !sorting.get() {
                            set_current_idx(None);
                            set_comparing_idx(None);
                            set_sorted_until(None);
                            break;
                        }

                        set_comparing_idx(Some(j - 1));
                        delay(100).await;

                        if current_array[j - 1] > current_array[j] {
                            current_array.swap(j - 1, j);
                            set_array(current_array.clone());
                            j -= 1;
                        } else {
                            break;
                        }
                        delay(100).await;
                    }
                    
                    // Only update sorted portion if still sorting
                    if sorting.get() {
                        set_sorted_until(Some(i));
                        delay(100).await;
                    }
                }
                
                if sorting.get() {
                    set_sorted_until(Some(len - 1));
                }
                
                set_sorting.set(false);
                set_current_idx(None);
                set_comparing_idx(None);
            },
        );
    };

    let stop_sorting = move |_: web_sys::MouseEvent| {
        set_sorting.set(false);
    };

    let on_size_change = move |ev| {
        let new_size = event_target_value(&ev)
            .parse::<usize>()
            .unwrap_or(50);
        set_array_size(new_size);
        generate_array(new_size);
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
            on_sort=insertion_sort
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
            <div class="h-full flex items-end gap-1">
                {move || {
                    array.get().into_iter().enumerate().map(|(idx, value)| {
                        let height = format!("{}%", value);
                        let is_current = current_idx.get().map(|i| idx == i).unwrap_or(false);
                        let is_comparing = comparing_idx.get().map(|i| idx == i).unwrap_or(false);
                        let is_sorted = sorted_until.get().map(|bound| idx <= bound).unwrap_or(false);
                        
                        let color = if is_current {
                            "#ef4444" 
                        } else if is_comparing {
                            "#22c55e"
                        } else if is_sorted {
                            "#e3963e"
                        } else {
                            "#6c6c6c"
                        };
                        
                        let scale = if is_current {
                            "transform: scaleY(1.1); transform-origin: bottom;"
                        } else {
                            "transform: scaleY(1.0); transform-origin: bottom;"
                        };
                        
                        view! {
                            <div
                                class="flex-1 transition-all duration-100"
                                style=move || format!(
                                    "height: {}; background-color: {}; {}",
                                    height,
                                    color,
                                    scale
                                )
                            >
                            </div>
                        }
                    }).collect_view()
                }}
            </div>
        </div>
        <AlgorithmInfoPanel algorithm_info=INSERTION_SORT_INFO/>
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