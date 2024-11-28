use leptos::*;

#[component]
pub fn NavBar() -> impl IntoView {
    let (is_open, set_is_open) = create_signal(false);

    let toggle_nav = move |_| {
        set_is_open.update(|open| *open = !*open);
    };

    view! {
        <nav class="bg-gray-800 fixed w-full z-50 top-0 left-0">
            <div class="max-w-7xl mx-auto px-4">
                <div class="flex justify-between h-16">
                    // Logo/Brand section
                    <div class="flex items-center">
                        <a href="/" class="text-white text-xl font-bold">
                            "Sorting Visualizer"
                        </a>
                    </div>

                    // Hamburger button
                    <div class="flex items-center sm:hidden">
                        <button
                            class="text-gray-300 hover:text-white p-2"
                            on:click=toggle_nav
                        >
                            <svg
                                class="h-6 w-6"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                {move || if !is_open() {
                                    view! {
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M4 6h16M4 12h16M4 18h16"
                                        />
                                    }
                                } else {
                                    view! {
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M6 18L18 6M6 6l12 12"
                                        />
                                    }
                                }}
                            </svg>
                        </button>
                    </div>

                    // Desktop navigation
                    <div class="hidden sm:flex sm:items-center">
                        <a
                            href="/selection-sort"
                            class="text-gray-300 hover:text-white px-3 py-2 rounded-md text-sm font-medium"
                        >
                            "Selection Sort"
                        </a>
                        <a
                            href="/bogo-sort"
                            class="text-gray-300 hover:text-white px-3 py-2 rounded-md text-sm font-medium"
                        >
                            "Bogo Sort"
                        </a>
                        <a
                            href="/bubble-sort"
                            class="text-gray-300 hover:text-white px-3 py-2 rounded-md text-sm font-medium"
                        >
                            "Bubble Sort"
                        </a>
                        <a
                            href="/quick-sort"
                            class="text-gray-300 hover:text-white px-3 py-2 rounded-md text-sm font-medium"
                        >
                            "Quick Sort"
                        </a>
                        <a
                            href="/merge-sort"
                            class="text-gray-300 hover:text-white px-3 py-2 rounded-md text-sm font-medium"
                        >
                            "Merge Sort"
                        </a>
                        <a
                            href="/insert-sort"
                            class="text-gray-300 hover:text-white px-3 py-2 rounded-md text-sm font-medium"
                        >
                            "Insertion Sort"
                        </a>
                        <a
                            href="/path-finding"
                            class="text-gray-300 hover:text-white px-3 py-2 rounded-md text-sm font-medium"
                        >
                            "PathFinding"
                        </a>
                    </div>
                </div>
            </div>

            // Mobile navigation
            {move || if is_open() {
                view! {
                    <div class="sm:hidden bg-gray-800">
                        <div class="px-2 pt-2 pb-3 space-y-1">
                            <a
                                href="/selection-sort"
                                class="text-gray-300 hover:text-white block px-3 py-2 rounded-md text-base font-medium"
                            >
                                "Selection Sort"
                            </a>
                            <a
                                href="/bogo-sort"
                                class="text-gray-300 hover:text-white block px-3 py-2 rounded-md text-base font-medium"
                            >
                                "Bogo Sort"
                            </a>
                            <a
                                href="/bubble-sort"
                                class="text-gray-300 hover:text-white block px-3 py-2 rounded-md text-base font-medium"
                            >
                                "Bubble Sort"
                            </a>
                            <a
                                href="/quick-sort"
                                class="text-gray-300 hover:text-white block px-3 py-2 rounded-md text-base font-medium"
                            >
                                "Quick Sort"
                            </a>
                            <a
                                href="/merge-sort"
                                class="text-gray-300 hover:text-white block px-3 py-2 rounded-md text-base font-medium"
                            >
                                "Merge Sort"
                            </a>
                            <a
                                href="/insert-sort"
                                class="text-gray-300 hover:text-white block px-3 py-2 rounded-md text-base font-medium"
                            >
                                "Insertion Sort"
                            </a>
                            <a
                                href="/path-finding"
                                class="text-gray-300 hover:text-white block px-3 py-2 rounded-md text-base font-medium"
                            >
                                "PathFinding"
                            </a>
                        </div>
                    </div>
                }
            } else {
                view! { <div></div> }
            }}
        </nav>

        <div class="h-16"></div>
    }
}