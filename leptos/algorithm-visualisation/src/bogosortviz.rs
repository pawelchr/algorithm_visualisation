use leptos::*;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::time::Duration;
use crate::navbar::NavBar;
use crate::sorting_controls::SortingControls;
use crate::sorting_info::{AlgorithmInfoPanel, BOGO_SORT_INFO};

#[component]
pub fn BogoSortVisualizer() -> impl IntoView {
    let (array, set_array) = create_signal(vec![]);
    let (sorting, set_sorting) = create_signal(false);
    let (array_size, set_array_size) = create_signal(4);
    let (attempts, set_attempts) = create_signal(0);
    
    let generate_array = move |size: usize| {
        let mut rng = thread_rng();
        let new_array: Vec<i32> = (0..size)
            .map(|_| rng.gen_range(10..100))
            .collect();
        set_array(new_array);
        set_attempts(0);
    };

    fn is_sorted(arr: &[i32]) -> bool {
        arr.windows(2).all(|w| w[0] <= w[1])
    }

    let bogo_sort = move |_: web_sys::MouseEvent| {
        set_sorting.set(true);
        set_attempts(0);
        
        create_local_resource(
            move || (),
            move |_| async move {
                let mut current_array = array.get();
                let mut rng = thread_rng();
                let mut attempt_count = 0;

                while !is_sorted(&current_array) && sorting.get() {
                    current_array.shuffle(&mut rng);
                    set_array(current_array.clone());
                    attempt_count += 1;
                    set_attempts(attempt_count);
                    
                    // Add delay to visualize the shuffling
                    delay(100).await;
                    
                    // Safety check to prevent infinite loops
                    if attempt_count > 10000 {
                        set_sorting.set(false);
                        break;
                    }
                }
                
                if !sorting.get() {
                    set_attempts(0);
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
            .unwrap_or(10)
            .min(15);
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
            on_sort=bogo_sort
            on_size_change=on_size_change
            min_size=3
            max_size=15
        >
            <button
                class="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 transition-colors"
                on:click=stop_sorting
                disabled=move || !sorting.get()
            >
                "Stop Sorting"
            </button>
        </SortingControls>
        
        <div class="w-full max-w-5xl mx-auto px-4">
            <div class="mb-4">
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
                            "#ef4444"
                        } else if is_sorted(&array.get()) {
                            "#22c55e"
                        } else {
                            "#6c6c6c"
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
        <AlgorithmInfoPanel algorithm_info=BOGO_SORT_INFO/>
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