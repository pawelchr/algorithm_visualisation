use leptos::*;
use crate::path_finding::{TraceStep, gen_field_graph, trace};
use std::rc::Rc;

#[component]
pub fn PathFinding() -> impl IntoView {
    let (grid_width, set_grid_width) = create_signal(32);
    let (grid_height, set_grid_height) = create_signal(32);
    let (start_index, set_start_index) = create_signal(0);
    let (end_index, set_end_index) = create_signal((grid_width.get() * grid_height.get() / 2) as u32);

    let update_grid_size = move || {
        let window = window();
        let width = window.inner_width().ok().and_then(|w| w.as_f64()).unwrap_or(800.0) as u32;
        let height = window.inner_height().ok().and_then(|h| h.as_f64()).unwrap_or(600.0) as u32;
        let new_width = width / 35;
        let new_height = (height - 200) / 30;
        set_grid_width.set(new_width);
        set_grid_height.set(new_height);
    };

    update_grid_size();

    window_event_listener(ev::resize, move |_| {
        update_grid_size();
    });

    let graph = create_memo(move |_| gen_field_graph(grid_width.get(), grid_height.get().into()));
    
    let trace = create_memo(move |_| {
        Rc::new(trace(start_index.get(), end_index.get(), &graph.get()))
    });
    
    view! {
        <div class="flex flex-col h-screen">
            <nav class="bg-gray-800 text-white p-4">
                <h1 class="text-xl font-bold">Path Finding Visualization</h1>
            </nav>
            <div class="flex-grow overflow-hidden">
                <TraceVis
                    width=grid_width
                    height=grid_height
                    trace=trace
                    start_index=start_index
                    end_index=end_index
                    on_start_change=set_start_index
                    on_end_change=set_end_index
                />
            </div>
        </div>
    }
}

#[component]
fn TraceVis(
    width: ReadSignal<u32>,
    height: ReadSignal<u32>,
    trace: Memo<Rc<Vec<TraceStep>>>,
    start_index: ReadSignal<u32>,
    end_index: ReadSignal<u32>,
    on_start_change: WriteSignal<u32>,
    on_end_change: WriteSignal<u32>,
) -> impl IntoView {
    let (current_step, set_current_step) = create_signal(0);
    let (is_animating, set_is_animating) = create_signal(false);
    let (selection_mode, set_selection_mode) = create_signal(SelectionMode::None);

    let default_step = create_memo(move |_| Rc::new(TraceStep {
        current: 0,
        frontier: vec![],
        visited: vec![0xffff; (width() * height()) as usize],
    }));

    let current_trace_step = create_memo(move |_| {
        trace.with(|t| t.get(current_step.get()).cloned().unwrap_or_else(|| default_step.get().as_ref().clone()))
    });

    let grid = move || {
        let step = current_trace_step.get();
        let w = width();
        let h = height();
        let start = start_index();
        let end = end_index();

        (0..h).flat_map(move |y| {
             let step_frontier = step.frontier.clone();
             let step_visited = step.visited.clone();
            (0..w).map(move |x| {
                let index = (y * w + x) as u32;
                let cell_class = if start == index {
                    "bg-blue-500"
                } else if end == index {
                    "bg-red-500"
                } else if index == step.current {
                    "bg-blue-500"
                } else if step_frontier.contains(&index) {
                    "bg-green-300"
                } else if step_visited.get(index as usize).map_or(false, |&v| v != 0xffff) {
                    "bg-gray-300"
                } else {
                    "bg-white"
                };
                view! {
                    <div
                        class=format!(
                            "w-8 h-8 border border-gray-400 flex items-center justify-center cursor-pointer {}",
                            cell_class,
                        )
                        on:click=move |_| {
                            match selection_mode.get() {
                                SelectionMode::Start => on_start_change.set(index),
                                SelectionMode::End => on_end_change.set(index),
                                SelectionMode::None => (),
                            }
                        }
                    />
                }
            })
        }).collect::<Vec<_>>()
    };

    let animate = move |_| {
        set_current_step.set(0);
        set_is_animating.set(true);

        let total_steps = trace.with(|t| t.len());
        let interval = set_interval_with_handle(
            move || {
                set_current_step.update(|step| {
                    if *step >= total_steps - 1 {
                        set_is_animating.set(false);
                    } else {
                        *step += 1;
                    }
                });
            },
            std::time::Duration::from_millis(50) // Increased interval for smoother animation
        );

        if let Ok(handle) = interval {
            create_effect(move |_| {
                if !is_animating() {
                    handle.clear();
                }
            });
        }
    };

    view! {
        <div class="flex flex-col items-center">
            <div class="mb-4">
                <button
                    class="px-4 py-2 bg-blue-500 text-white rounded mr-2"
                    on:click=move |_| set_selection_mode.set(SelectionMode::Start)
                >
                    "Select Start"
                </button>
                <button
                    class="px-4 py-2 bg-red-500 text-white rounded"
                    on:click=move |_| set_selection_mode.set(SelectionMode::End)
                >
                    "Select End"
                </button>
            </div>
            <div class="grid" style=move || format!("grid-template-columns: repeat({}, 1fr);", width())>
                {grid}
            </div>
            <div class="mt-4 flex items-center">
                <button
                    class="px-4 py-2 bg-blue-500 text-white rounded mr-2"
                    on:click=move |_| set_current_step.update(|s| *s = s.saturating_sub(1))
                >
                    "Previous"
                </button>
                <span class="mx-2">
                    {"Step "} {move || current_step.get() + 1} {" of "} {move || trace.with(|t| t.len())}
                </span>
                <button
                    class="px-4 py-2 bg-blue-500 text-white rounded ml-2"
                    on:click=move |_| set_current_step.update(|s| *s = (*s + 1).min(trace.with(|t| t.len().saturating_sub(1))))
                >
                    "Next"
                </button>
                <button
                    class="ml-2 px-4 py-2 bg-green-500 text-white rounded"
                    on:click=animate
                    disabled=move || is_animating()
                >
                    {move || if is_animating() { "Animating..." } else { "Animate" }}
                </button>
            </div>
        </div>
    }
}

#[derive(Clone, Copy, PartialEq)]
enum SelectionMode {
    None,
    Start,
    End,
}