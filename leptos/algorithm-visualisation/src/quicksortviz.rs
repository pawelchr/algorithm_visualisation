use leptos::*;
use rand::Rng;
use std::time::Duration;
use std::pin::Pin;
use std::future::Future;
use crate::navbar::NavBar;

#[component]
pub fn QuickSortVisualizer() -> impl IntoView {
    let (array, set_array) = create_signal(vec![]);
    let (sorting, set_sorting) = create_signal(false);
    let (pivot_idx, set_pivot_idx) = create_signal(None::<usize>);
    let (comparing_indices, set_comparing_indices) = create_signal(Vec::new());
    let (array_size, set_array_size) = create_signal(50);
    let (sorted_indices, set_sorted_indices) = create_signal(Vec::new());
    
    // Initialize array with random values
    let generate_array = move |size: usize| {
        let mut rng = rand::thread_rng();
        let new_array: Vec<i32> = (0..size)
            .map(|_| rng.gen_range(10..100))
            .collect();
        set_array(new_array);
        set_pivot_idx(None);
        set_comparing_indices(Vec::new());
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

    async fn partition_async(
        array: &mut Vec<i32>,
        low: isize,
        high: isize,
        set_array: WriteSignal<Vec<i32>>,
        set_pivot_idx: WriteSignal<Option<usize>>,
        set_comparing_indices: WriteSignal<Vec<usize>>,
    ) -> isize {
        let pivot = array[high as usize];
        set_pivot_idx(Some(high as usize));
        
        let mut i = low - 1;
        
        for j in low..high {
            set_comparing_indices(vec![j as usize, high as usize]);
            delay(50).await;
            
            if array[j as usize] <= pivot {
                i += 1;
                array.swap(i as usize, j as usize);
                set_array(array.clone());
                delay(50).await;
            }
        }
        
        let pivot_pos = i + 1;
        array.swap(pivot_pos as usize, high as usize);
        set_array(array.clone());
        delay(50).await;
        
        pivot_pos
    }

    fn quick_sort_async<'a>(
        array: &'a mut Vec<i32>,
        low: isize,
        high: isize,
        set_array: WriteSignal<Vec<i32>>,
        set_pivot_idx: WriteSignal<Option<usize>>,
        set_comparing_indices: WriteSignal<Vec<usize>>,
        sorted_indices: ReadSignal<Vec<usize>>,
        set_sorted_indices: WriteSignal<Vec<usize>>,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
        Box::pin(async move {
            if low < high {
                let pivot = partition_async(
                    array,
                    low,
                    high,
                    set_array,
                    set_pivot_idx,
                    set_comparing_indices,
                ).await;
                
                // Mark pivot as sorted
                let mut sorted = sorted_indices.get();
                sorted.push(pivot as usize);
                set_sorted_indices(sorted);
                
                // Recursively sort the sub-arrays
                quick_sort_async(
                    array, 
                    low, 
                    pivot - 1, 
                    set_array, 
                    set_pivot_idx, 
                    set_comparing_indices,
                    sorted_indices,
                    set_sorted_indices,
                ).await;
                
                quick_sort_async(
                    array, 
                    pivot + 1, 
                    high, 
                    set_array, 
                    set_pivot_idx, 
                    set_comparing_indices,
                    sorted_indices,
                    set_sorted_indices,
                ).await;
            } else if low == high {
                // Single element is always sorted
                let mut sorted = sorted_indices.get();
                sorted.push(low as usize);
                set_sorted_indices(sorted);
            }
        })
    }

    // QuickSort implementation with visualization
    let quick_sort = move |_| {
        set_sorting.set(true);
        set_sorted_indices(Vec::new());
        
        create_local_resource(
            move || (),
            move |_| async move {
                let mut current_array = array.get();
                let len = current_array.len();
                
                quick_sort_async(
                    &mut current_array,
                    0,
                    (len - 1) as isize,
                    set_array,
                    set_pivot_idx,
                    set_comparing_indices,
                    sorted_indices,
                    set_sorted_indices,
                ).await;
                
                // Mark all elements as sorted at the end
                set_sorted_indices((0..len).collect());
                set_sorting.set(false);
                set_pivot_idx(None);
                set_comparing_indices(Vec::new());
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
                        on:click=quick_sort
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
            
            <div class="h-96 flex items-end gap-1">
                {move || {
                    array.get().into_iter().enumerate().map(|(idx, value)| {
                        let height = format!("{}%", value);
                        let is_pivot = pivot_idx.get().map(|i| idx == i).unwrap_or(false);
                        let is_comparing = comparing_indices.get().contains(&idx);
                        let is_sorted = sorted_indices.get().contains(&idx);
                        
                        let color = if is_pivot {
                            "#ef4444"  // Red for pivot
                        } else if is_comparing {
                            "#22c55e"  // Green for comparing
                        } else if is_sorted {
                            "#e3963e"  // Orange for sorted
                        } else {
                            "#6c6c6c"  // Gray for unsorted
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