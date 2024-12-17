use leptos::*;
use crate::navbar::NavBar;

#[derive(Clone)]
struct Algorithm {
    title: &'static str,
    description: &'static str,
    path: &'static str,
    complexity: &'static str,
    icon: &'static str,
}

#[component]
pub fn LandingPage() -> impl IntoView {
    let algorithms = vec![
        Algorithm {
            title: "Bubble Sort",
            description: "Simple sorting algorithm that repeatedly steps through the list, compares adjacent elements and swaps them if they are in the wrong order.",
            path: "/bubble-sort",
            complexity: "O(n²)",
            icon: "↕️",
        },
        Algorithm {
            title: "Quick Sort",
            description: "Efficient divide-and-conquer sorting algorithm that partitions data around a pivot, recursively sorting the sub-arrays.",
            path: "/quick-sort",
            complexity: "O(n log n)",
            icon: "🔄",
        },
        Algorithm {
            title: "Merge Sort",
            description: "Stable divide-and-conquer sorting algorithm that divides the array into smaller subarrays, sorts, and then merges them.",
            path: "/merge-sort",
            complexity: "O(n log n)",
            icon: "🔀",
        },
        Algorithm {
            title: "Insertion Sort",
            description: "Simple sorting algorithm that builds the final sorted array one item at a time, efficient for small data sets.",
            path: "/insert-sort",
            complexity: "O(n²)",
            icon: "➡️",
        },
        Algorithm {
            title: "Selection Sort",
            description: "Sorting algorithm that divides input into sorted and unsorted regions, repeatedly selecting the smallest element.",
            path: "/selection-sort",
            complexity: "O(n²)",
            icon: "⬆️",
        },
        Algorithm {
            title: "Bogo Sort",
            description: "A highly inefficient sorting algorithm that randomly shuffles elements until they are sorted. For educational purposes only!",
            path: "/bogo-sort",
            complexity: "O(n × n!)",
            icon: "🎲",
        },
        Algorithm {
            title: "Pathfinding Visualizer",
            description: "Visualize different pathfinding algorithms including Dijkstra's, A*, BFS, DFS, and more.",
            path: "/path-finding",
            complexity: "Various",
            icon: "🗺️",
        },
    ];

    view! {
        <NavBar/>
        
        // Hero Section
        <div class="bg-white shadow-sm">
            <div class="max-w-7xl mx-auto px-4 py-16">
                <div class="text-center">
                    <h1 class="text-4xl font-bold text-gray-900 sm:text-5xl md:text-6xl">
                        "Algorithm Visualizer"
                    </h1>
                    <p class="mt-6 max-w-2xl mx-auto text-xl text-gray-500">
                        "Interactive visualizations of sorting algorithms and pathfinding techniques. Learn how different algorithms work through animated demonstrations."
                    </p>
                </div>
            </div>
        </div>

        // Algorithm Cards Grid
        <div class="max-w-7xl mx-auto px-4 py-12">
            <div class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
                {algorithms.into_iter().map(|algo| {
                    view! {
                        <a 
                            href={algo.path}
                            class="transform transition-all duration-200 hover:scale-105"
                        >
                            <div class="bg-white rounded-lg shadow-md p-6 h-full hover:shadow-lg">
                                <div class="flex items-center justify-between mb-4">
                                    <div class="text-4xl">
                                        {algo.icon}
                                    </div>
                                    <span class="text-sm font-mono bg-gray-100 px-2 py-1 rounded">
                                        {algo.complexity}
                                    </span>
                                </div>
                                <h3 class="text-xl font-semibold mb-2">
                                    {algo.title}
                                </h3>
                                <p class="text-gray-600">
                                    {algo.description}
                                </p>
                            </div>
                        </a>
                    }
                }).collect_view()}
            </div>
        </div>

        // Footer
        <footer class="bg-white mt-12">
            <div class="max-w-7xl mx-auto py-12 px-4">
                <div class="text-center text-gray-500">
                    <p>"Built with Leptos and Rust • Visualize and learn algorithms interactively"</p>
                </div>
            </div>
        </footer>
    }
}