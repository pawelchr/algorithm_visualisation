use leptos::*;
use rand::Rng;
use std::time::Duration;
use crate::navbar::NavBar;

#[component]
pub fn BubbleSortVisualizer() -> impl IntoView {
    let (array, set_array) = create_signal(vec![]);
    let (sorting, set_sorting) = create_signal(false);
    let (comparison_idx, set_comparison_idx) = create_signal(None::<(usize, usize)>);
    let (array_size, set_array_size) = create_signal(50);
    let (sorted_indices, set_sorted_indices) = create_signal(Vec::new());
    
    // Initialize array with random values
    let generate_array = move |size: usize| {
        let mut rng = rand::thread_rng();
        let new_array: Vec<i32> = (0..size)
            .map(|_| rng.gen_range(10..100))
            .collect();
        set_array(new_array);
        set_comparison_idx(None);
        set_sorted_indices(Vec::new());
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

    // Bubble sort implementation with visualization
    let bubble_sort = move |_| {
        set_sorting.set(true);
        set_sorted_indices(Vec::new());
        
        create_local_resource(
            move || (),
            move |_| async move {
                let mut current_array = array.get();
                let len = current_array.len();
                let mut sorted = Vec::new();

                for i in 0..len {
                    let mut swapped = false;
                    for j in 0..len - i - 1 {
                        set_comparison_idx.set(Some((j, j + 1)));
                        
                        if current_array[j] > current_array[j + 1] {
                            current_array.swap(j, j + 1);
                            set_array.set(current_array.clone());
                            swapped = true;
                        }
                        
                        delay(50).await;
                    }
                    
                    // Add the last element of this pass to sorted
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
                
                // Ensure all indices are marked as sorted at the end
                set_sorted_indices((0..len).collect());
                set_sorting.set(false);
                set_comparison_idx.set(None);
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
                        on:click=bubble_sort
                        disabled=sorting
                    >
                        "Start Sorting"
                    </button>
                    <button
                        class="px-4 py-2 bg-red-500 text-white rounded"
                        on:click=bubble_sort
                        disabled=sorting
                    >
                        "Stop Sorting"
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
            
            <div class="h-96 flex items-end gap-1">
                {move || {
                    array.get().into_iter().enumerate().map(|(idx, value)| {
                        let height = format!("{}%", value);
                        let is_comparing = comparison_idx.get()
                            .map(|(i, j)| idx == i || idx == j)
                            .unwrap_or(false);
                        let is_sorted = sorted_indices.get().contains(&idx);
                        
                        let color = if is_comparing {
                            "#22c55e"  // Red for comparing
                        } else if is_sorted {
                            "#e3963e"  // Orange for sorted
                        } else {
                            "#6c6c6c"  // Blue for unsorted
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
        </div>
    }
}