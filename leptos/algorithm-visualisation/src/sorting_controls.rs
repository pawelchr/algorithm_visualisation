use ev::{Event, MouseEvent};
use leptos::*;

#[component]
pub fn SortingControls(
    #[prop(into)] array_size: Signal<usize>,
    #[prop(into)] is_sorting: Signal<bool>,
    #[prop(into)] on_generate: Callback<MouseEvent>,
    #[prop(into)] on_sort: Callback<MouseEvent>,
    #[prop(into)] on_size_change: Callback<Event>,
    #[prop(default = 10)] min_size: usize,
    #[prop(default = 150)] max_size: usize,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div class="w-full max-w-7xl mx-auto p-4">
            <div class="mb-4 flex flex-col gap-4">
                <div class="flex items-center gap-4 flex-wrap">
                    <button
                        class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
                        on:click=on_generate
                        disabled=is_sorting
                    >
                        "Generate New Array"
                    </button>
                    <button
                        class="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 transition-colors"
                        on:click=on_sort
                        disabled=is_sorting
                    >
                        "Start Sorting"
                    </button>
                    {children.map(|c| c())}
                </div>
                <div class="flex items-center gap-4">
                    <label class="whitespace-nowrap">"Array Size: " {move || array_size.get().to_string()}</label>
                    <input 
                        type="range"
                        min=min_size
                        max=max_size
                        value=array_size
                        class="w-64"
                        on:input=on_size_change
                        disabled=is_sorting
                    />
                </div>
            </div>
        </div>
    }
}