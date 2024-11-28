use leptos::*;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::time::Duration;
use crate::navbar::NavBar;

#[component]
pub fn BogoSortVisualizer() -> impl IntoView {
    let (array, set_array) = create_signal(vec![]);
    let (sorting, set_sorting) = create_signal(false);
    let (array_size, set_array_size) = create_signal(10); // Smaller default size since bogosort is very inefficient
    let (attempts, set_attempts) = create_signal(0);

    // Initialize array with random values
    let generate_array = move |size: usize| {
        let mut rng = thread_rng();
        let new_array: Vec<i32> = (0..size)
            .map(|_| rng.gen_range(10..100))
            .collect();
        set_array(new_array);
        set_attempts(0);
    };

    // Helper function to create a delay
    fn delay(ms: u64) -> impl std::future::Future<Output = ()> {
        async move {
            let (tx, rx) = futures::channel::oneshot::channel::<()>();
            set_timeout(move || {
                let _ = tx.send(());
            }, Duration::from_millis(ms));
            let _ = rx.await;
        }
    }

    // Check if array is sorted
    fn is_sorted(arr: &[i32]) -> bool {
        arr.windows(2).all(|w| w[0] <= w[1])
    }

    // BogoSort implementation with visualization
    let bogo_sort = move |_| {
        set_sorting.set(true);
        set_attempts(0);
        
        create_local_resource(
            move || (),
            move |_| async move {
                let mut current_array = array.get();
                let mut rng = thread_rng();
                let mut attempt_count = 0;

                while !is_sorted(&current_array) {
                    // Shuffle the array
                    current_array.shuffle(&mut rng);
                    set_array(current_array.clone());
                    attempt_count += 1;
                    set_attempts(attempt_count);
                    
                    // Add delay to visualize the shuffling
                    delay(100).await;
                    
                    // Safety check to prevent infinite loops
                    if attempt_count > 10000 {
                        break;
                    }
                }
                
                set_sorting.set(false);
            },
        );
    };

    // Handle size change
    let on_size_change = move |ev| {
        let new_size = event_target_value(&ev)
            .parse::<usize>()
            .unwrap_or(10)
            .min(15); // Limit maximum size since bogosort is very inefficient
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
                        on:click=bogo_sort
                        disabled=sorting
                    >
                        "Start Sorting"
                    </button>
                </div>
                <div class="flex items-center gap-4">
                    <label>"Array Size: " {move || array_size().to_string()}</label>
                    <input 
                        type="range"
                        min="3"
                        max="15"
                        value={array_size}
                        class="w-64"
                        on:input=on_size_change
                        disabled=sorting
                    />
                </div>
                <div>
                    "Shuffle Attempts: " {move || attempts().to_string()}
                </div>
            </div>
            
            <div class="relative h-96">
                <div class="h-full flex items-end gap-1">
                    {move || {
                        array.get().into_iter().enumerate().map(|(_, value)| {
                            let height = format!("{}%", value);
                            let is_sorting = sorting();
                            
                            let color = if is_sorting {
                                "#ef4444"  // Red while sorting
                            } else if is_sorted(&array.get()) {
                                "#22c55e"  // Green when sorted
                            } else {
                                "#6c6c6c"  // Gray otherwise
                            };
                            
                            view! {
                                <div
                                    class="flex-1 transition-all duration-100"
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
        </div>
    }
}