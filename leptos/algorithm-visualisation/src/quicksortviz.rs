use leptos::*;
use rand::Rng;
use std::time::Duration;
use std::pin::Pin;
use std::future::Future;
use crate::navbar::NavBar;
use crate::sorting_controls::SortingControls;
use crate::sorting_info::{AlgorithmInfoPanel, QUICK_SORT_INFO};

#[component]
pub fn QuickSortVisualizer() -> impl IntoView {
    let (array, set_array) = create_signal(vec![]);
    let (sorting, set_sorting) = create_signal(false);
    let (pivot_idx, set_pivot_idx) = create_signal(None::<usize>);
    let (comparing_indices, set_comparing_indices) = create_signal(Vec::new());
    let (array_size, set_array_size) = create_signal(20);
    let (sorted_indices, set_sorted_indices) = create_signal(Vec::new());
    
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

    fn is_sorted(arr: &[i32]) -> bool {
        arr.windows(2).all(|w| w[0] <= w[1])
    }

    async fn partition_async(
        array: &mut Vec<i32>,
        low: isize,
        high: isize,
        set_array: WriteSignal<Vec<i32>>,
        set_pivot_idx: WriteSignal<Option<usize>>,
        set_comparing_indices: WriteSignal<Vec<usize>>,
        sorting: ReadSignal<bool>,
    ) -> Option<isize> {
        if !sorting.get() {
            return None;
        }

        // Use median-of-three pivot selection
        let mid = low + (high - low) / 2;
        
        // Sort low, mid, high values
        if array[low as usize] > array[mid as usize] {
            array.swap(low as usize, mid as usize);
        }
        if array[mid as usize] > array[high as usize] {
            array.swap(mid as usize, high as usize);
        }
        if array[low as usize] > array[mid as usize] {
            array.swap(low as usize, mid as usize);
        }
        
        // Use middle value as pivot
        array.swap(mid as usize, high as usize);
        
        let pivot = array[high as usize];
        set_pivot_idx(Some(high as usize));
        
        let mut i = low - 1;
        
        for j in low..high {
            if !sorting.get() {
                return None;
            }

            set_comparing_indices(vec![j as usize, high as usize]);
            delay(30).await;
            
            if array[j as usize] <= pivot {
                i += 1;
                array.swap(i as usize, j as usize);
                set_array(array.clone());
                delay(30).await;
            }
        }
        
        let pivot_pos = i + 1;
        array.swap(pivot_pos as usize, high as usize);
        set_array(array.clone());
        delay(30).await;
        
        Some(pivot_pos)
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
        sorting: ReadSignal<bool>,
    ) -> Pin<Box<dyn Future<Output = bool> + 'a>> {
        Box::pin(async move {
            if !sorting.get() {
                return false;
            }

            // Early exit if segment is already sorted
            if low < high {
                let segment = &array[low as usize..=high as usize];
                if is_sorted(segment) {
                    let mut sorted = sorted_indices.get();
                    for i in low..=high {
                        if !sorted.contains(&(i as usize)) {
                            sorted.push(i as usize);
                        }
                    }
                    set_sorted_indices(sorted);
                    return true;
                }

                match partition_async(
                    array,
                    low,
                    high,
                    set_array,
                    set_pivot_idx,
                    set_comparing_indices,
                    sorting,
                ).await {
                    Some(pivot) => {
                        // Mark pivot as sorted
                        let mut sorted = sorted_indices.get();
                        sorted.push(pivot as usize);
                        set_sorted_indices(sorted);
                        
                        // Sort left partition
                        let left_completed = quick_sort_async(
                            array, 
                            low, 
                            pivot - 1,
                            set_array,
                            set_pivot_idx,
                            set_comparing_indices,
                            sorted_indices,
                            set_sorted_indices,
                            sorting,
                        ).await;

                        if !left_completed {
                            return false;
                        }
                        
                        // Sort right partition
                        quick_sort_async(
                            array,
                            pivot + 1,
                            high,
                            set_array,
                            set_pivot_idx,
                            set_comparing_indices,
                            sorted_indices,
                            set_sorted_indices,
                            sorting,
                        ).await
                    }
                    None => false
                }
            } else {
                if low >= 0 && low < array.len() as isize {
                    let mut sorted = sorted_indices.get();
                    if !sorted.contains(&(low as usize)) {
                        sorted.push(low as usize);
                        set_sorted_indices(sorted);
                    }
                }
                true
            }
        })
    }

    let quick_sort = move |_: web_sys::MouseEvent| {
        set_sorting.set(true);
        set_sorted_indices(Vec::new());
        
        create_local_resource(
            move || (),
            move |_| async move {
                let mut current_array = array.get();
                let len = current_array.len();
                
                let completed = quick_sort_async(
                    &mut current_array,
                    0,
                    (len - 1) as isize,
                    set_array,
                    set_pivot_idx,
                    set_comparing_indices,
                    sorted_indices,
                    set_sorted_indices,
                    sorting,
                ).await;
                
                if completed {
                    // Clear comparing indices and pivot when sorting is complete
                    set_comparing_indices(Vec::new());
                    set_pivot_idx(None);
                    set_sorted_indices((0..len).collect());
                } else {
                    // Clean up visualization state if cancelled
                    set_pivot_idx(None);
                    set_comparing_indices(Vec::new());
                    set_sorted_indices(Vec::new());
                }
                
                set_sorting.set(false);
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
            on_sort=quick_sort
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
                    let is_pivot = pivot_idx.get().map(|i| idx == i).unwrap_or(false);
                    let is_comparing = comparing_indices.get().contains(&idx);
                    let is_sorted = sorted_indices.get().contains(&idx);
                    
                    let color = if is_pivot {
                        "#ef4444"
                    } else if is_comparing {
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
        <AlgorithmInfoPanel algorithm_info=QUICK_SORT_INFO/>
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