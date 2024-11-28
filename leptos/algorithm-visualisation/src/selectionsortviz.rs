use leptos::*;
use rand::Rng;
use std::time::Duration;
use crate::navbar::NavBar;

#[component]
pub fn SelectionSortVisualizer() -> impl IntoView {
    let (array, set_array) = create_signal(vec![]);
    let (sorting, set_sorting) = create_signal(false);
    let (current_min_idx, set_current_min_idx) = create_signal(None::<usize>);
    let (checking_idx, set_checking_idx) = create_signal(None::<usize>);
    let (array_size, set_array_size) = create_signal(50);
    let (sorted_until, set_sorted_until) = create_signal(None::<usize>);
    
    // Initialize array with random values
    let generate_array = move |size: usize| {
        let mut rng = rand::thread_rng();
        let new_array: Vec<i32> = (0..size)
            .map(|_| rng.gen_range(10..100))
            .collect();
        set_array(new_array);
        set_current_min_idx(None);
        set_checking_idx(None);
        set_sorted_until(None);
    };

    // Helper function to create a promise-like delay
    fn delay(ms: u64) -> impl std::future::Future<Output = ()> {
        async move {
            let (tx, rx) = futures::channel::oneshot::channel::<()>();
            set_timeout(move || {
                let _ = tx.send(());
            }, Duration::from_millis(ms));
            let _ = rx.await;
        }
    }

    // SelectionSort implementation with visualization
    let selection_sort = move |_| {
        set_sorting.set(true);
        set_sorted_until(None);
        
        create_local_resource(
            move || (),
            move |_| async move {
                let mut current_array = array.get();
                let len = current_array.len();

                for i in 0..len {
                    let mut min_idx = i;
                    set_current_min_idx(Some(min_idx));
                    delay(200).await;

                    // Find the minimum element in the unsorted portion
                    for j in (i + 1)..len {
                        set_checking_idx(Some(j));
                        delay(100).await;

                        if current_array[j] < current_array[min_idx] {
                            min_idx = j;
                            set_current_min_idx(Some(min_idx));
                            delay(100).await;
                        }
                    }

                    // Swap the minimum element with the first unsorted element
                    if min_idx != i {
                        current_array.swap(i, min_idx);
                        set_array(current_array.clone());
                        delay(200).await;
                    }

                    set_sorted_until(Some(i));
                    set_checking_idx(None);
                    delay(200).await;
                }
                
                // Mark all elements as sorted at the end
                set_sorted_until(Some(len - 1));
                set_sorting.set(false);
                set_current_min_idx(None);
                set_checking_idx(None);
            },
        );
    };

    // Handle size change
    let on_size_change = move |ev| {
        let new_size = event_target_value(&ev)
            .parse::<usize>()
            .unwrap_or(50);
        set_array_size(new_size);
        generate_array(new_size);
    };

    // Initial array generation
    create_effect(move |_| {
        generate_array(array_size());
    });

    view! {
        <NavBar/>
        <div class="w-full max-w-5xl mx-auto p-4">
            <div class="mb-4 flex flex-col gap-4">
                <div class="flex items-center gap-4">
                    <button
                        class="px-4 py-2 bg-blue-500 text-white rounded"
                        on:click=move |_| generate_array(array_size())
                        disabled=sorting
                    >
                        "Generate New Array"
                    </button>
                    <button
                        class="px-4 py-2 bg-green-500 text-white rounded"
                        on:click=selection_sort
                        disabled=sorting
                    >
                        "Start Sorting"
                    </button>
                </div>
                <div class="flex items-center gap-4">
                    <label>"Array Size: " {move || array_size().to_string()}</label>
                    <input 
                        type="range"
                        min="10"
                        max="150"
                        value={array_size}
                        class="w-64"
                        on:input=on_size_change
                        disabled=sorting
                    />
                </div>
            </div>
            
            <div class="relative h-96">
                <div class="h-full flex items-end gap-1">
                    {move || {
                        array.get().into_iter().enumerate().map(|(idx, value)| {
                            let height = format!("{}%", value);
                            let is_min = current_min_idx.get().map(|i| idx == i).unwrap_or(false);
                            let is_checking = checking_idx.get().map(|i| idx == i).unwrap_or(false);
                            let is_sorted = sorted_until.get().map(|bound| idx <= bound).unwrap_or(false);
                            
                            let color = if is_min {
                                "#ef4444"  // Red for current minimum
                            } else if is_checking {
                                "#22c55e"  // Green for element being checked
                            } else if is_sorted {
                                "#e3963e"  // Orange for sorted portion
                            } else {
                                "#6c6c6c"  // Gray for unsorted
                            };
                            
                            let scale = if is_min {
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
        </div>
    }
}