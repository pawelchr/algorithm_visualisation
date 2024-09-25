use core::time;
use std::thread::sleep;

use html::Table;
use leptos::*;
use leptos_struct_table::*;
use web_sys::Window;
use crate::path_finding::{TraceStep, gen_field_graph, trace};


#[component]
pub fn PathFinding() -> impl IntoView {
    let (grid_width, set_grid_width) = create_signal(32);
    let (grid_height, set_grid_height) = create_signal(32);

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
    let end_point = create_memo(move |_| (grid_width.get() * grid_height.get() / 2) as u32);
    let trace = create_memo(move |_| trace(0, end_point.get(), &graph.get()));
    
    view! {
        <div class="flex flex-col h-screen">
            <nav class="bg-gray-800 text-white p-4">
                <h1 class="text-xl font-bold">Path Finding Visualization</h1>
            </nav>
            <div class="flex-grow overflow-hidden">
                <TraceVis
                    width=grid_width()
                    height=grid_height()
                    trace=trace()
                    start_index=Signal::derive(move || 0)()
                    end_index=end_point()
                />
            </div>
        </div>
    }
}


#[component]
fn TraceVis(width: u32, height: u32, trace: Vec<TraceStep>, start_index: u32, end_index: u32) -> impl IntoView {
    let (current_step, set_current_step) = create_signal(0);
    let total_steps = trace.len();
    let (is_animating, set_is_animating) = create_signal(false);
    let input_element: NodeRef<html::Input> = create_node_ref();
    let default_step = create_memo(move |_| TraceStep {
        current: 0,
        frontier: vec![],
        visited: vec![0xffff; (width * height) as usize],
    });
    let (height_reactive, set_height) = create_signal(height);
    let (width_reactive, set_width) = create_signal(width);

    let grid = move || {
        let binding = default_step().clone();
        let step = trace.get(current_step.get()).unwrap_or(&binding);

        (0..height_reactive()).map(|y| {
            (0..width_reactive()).map(|x| {
                let index = (y * width_reactive() + x) as u32;
                let cell_class = if start_index == index {
                    "bg-blue-500"
                }
                else if end_index == index {
                    "bg-red-500"
                }
                else if index == step.current {
                    "bg-blue-500"
                } else if step.frontier.contains(&index) {
                    "bg-green-300"
                } else if step.visited.get(index as usize).map_or(false, |&v| v != 0xffff) {
                    "bg-gray-300"
                } else {
                    "bg-white"
                };
                view! {
                    <div
                        class=format!(
                            "w-8 h-8 border border-gray-400 flex items-center justify-center {}",
                            cell_class,
                        )
                        id=format!("x-{}-y-{}", x, y)
                    >
                    </div>
                }
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>()
    };

    let animate = move |_| {
        set_current_step.set(0);
        set_is_animating.set(true);

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
            std::time::Duration::from_millis(1)
        );

        if let Ok(handle) = interval {
            create_effect(move |_| {
                if !is_animating() {
                    handle.clear();
                }
            });
        }
    };

    let submit_height = move |event: leptos::ev::SubmitEvent| {
        event.prevent_default();

        let value = input_element
            .get()
            .expect("<input> should be mounted")
            .value();
        set_height(value.parse::<u32>().unwrap())
    };
    let submit_width = move |event: leptos::ev::SubmitEvent| {
        event.prevent_default();

        let value = input_element
            .get()
            .expect("<input> should be mounted")
            .value();
        set_width(value.parse::<u32>().unwrap())
    };

    view! {
        <div class="flex flex-col items-center">
            <div class="grid" style=format!("grid-template-columns: repeat({}, 1fr);", width_reactive())>
                {grid}
            </div>
            <div class="mt-4 flex items-center">
                <button
                    class="px-4 py-2 bg-blue-500 text-white rounded mr-2"
                    on:click=move |_| {
                        set_current_step.set(current_step().saturating_sub(1));
                    }
                >
                    "Previous"
                </button>
                <span class="mx-2">
                    {"Step "} {move || current_step.get() + 1} {" of "} {total_steps}
                </span>
                <button
                    class="px-4 py-2 bg-blue-500 text-white rounded ml-2"
                    on:click=move |_| {
                        set_current_step
                            .set((current_step() + 1).min(total_steps.saturating_sub(1)));
                    }
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
                <form on:submit=submit_height>
                    <input 
                    type="number"
                    step=1
                    min=1
                    max=height
                    value=height_reactive
                    node_ref=input_element

                    />
                    <input 
                    type="submit"
                    value="Submit"
                    />
                </form>
                <form on:submit=submit_width>
                    <input 
                    type="number"
                    step=1
                    min=1
                    max=width
                    value=width_reactive
                    node_ref=input_element

                    />
                    <input 
                    type="submit"
                    value="Submit"
                    />
                </form>
            </div>
        </div>
    }
}