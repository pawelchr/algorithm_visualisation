use leptos::*;

#[derive(Clone)]
pub struct AlgorithmInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub best_time: &'static str,
    pub average_time: &'static str,
    pub worst_time: &'static str,
    pub space: &'static str,
    pub applications: &'static str,
}

pub const BUBBLE_SORT_INFO: AlgorithmInfo = AlgorithmInfo {
    name: "Bubble Sort",
    description: "Bubble Sort is a simple sorting algorithm that repeatedly steps through the list, compares adjacent elements and swaps them if they are in the wrong order. The algorithm gets its name because smaller elements 'bubble' to the top of the list with each iteration. This algorithm is mainly used for educational purposes and is practical only for small data sets where simple implementation is more important than efficiency. Its straightforward nature makes it an excellent teaching tool for introducing the concept of sorting algorithms.",
    best_time: "O(n)",
    average_time: "O(n²)",
    worst_time: "O(n²)",
    space: "O(1)",
    applications: "Educational purposes and small datasets",
};

pub const SELECTION_SORT_INFO: AlgorithmInfo = AlgorithmInfo {
    name: "Selection Sort",
    description: "Selection Sort divides the input into a sorted and an unsorted region. It repeatedly finds the minimum element from the unsorted region and adds it to the end of the sorted region. Like Bubble Sort, it's primarily used for educational purposes and small datasets where minimizing the number of swaps is important. It performs the same number of comparisons regardless of the initial order of elements, making it inefficient for large datasets.",
    best_time: "O(n²)",
    average_time: "O(n²)",
    worst_time: "O(n²)",
    space: "O(1)",
    applications: "Small datasets and educational purposes",
};

pub const INSERTION_SORT_INFO: AlgorithmInfo = AlgorithmInfo {
    name: "Insertion Sort",
    description: "Insertion Sort builds the final sorted array one item at a time by iterating through the input array and shifting larger elements to the right to make room for the current element being inserted. It's highly efficient for small and nearly sorted arrays. The algorithm's performance significantly improves when dealing with partially sorted data, making it a practical choice for maintaining sorted lists or sorting small chunks of data within larger algorithms.",
    best_time: "O(n)",
    average_time: "O(n²)",
    worst_time: "O(n²)",
    space: "O(1)",
    applications: "Small datasets and nearly sorted arrays",
};

pub const QUICK_SORT_INFO: AlgorithmInfo = AlgorithmInfo {
    name: "Quick Sort",
    description: "Quick Sort is based on the divide-and-conquer strategy. It selects a 'pivot' element and partitions the array around it, with smaller elements going to one side and larger elements to the other. The pivot then goes to its final position, and the process is recursively repeated for both sub-arrays. This algorithm is widely used in practice due to its efficient average-case performance and in-place sorting capability.",
    best_time: "O(n log n)",
    average_time: "O(n log n)",
    worst_time: "O(n²)",
    space: "O(log n)",
    applications: "General-purpose sorting, standard library implementations",
};

pub const MERGE_SORT_INFO: AlgorithmInfo = AlgorithmInfo {
    name: "Merge Sort",
    description: "Merge Sort divides the array into halves down to single elements, then merges these parts back together in sorted order. During merging, it compares elements from both parts and combines them in order, creating increasingly larger sorted segments. It guarantees consistent performance regardless of input order and is particularly efficient for large datasets. The algorithm is stable and predictable but requires additional memory space.",
    best_time: "O(n log n)",
    average_time: "O(n log n)",
    worst_time: "O(n log n)",
    space: "O(n)",
    applications: "Large datasets, external sorting, stable sorting requirements",
};

pub const BOGO_SORT_INFO: AlgorithmInfo = AlgorithmInfo {
    name: "Bogo Sort",
    description: "Bogo Sort is a highly inefficient sorting algorithm that works by randomly shuffling elements and checking if they are sorted. If not, it repeats the process. It serves as an educational example of how not to design algorithms. While there's always a minimal chance it might sort correctly on the first shuffle, it's completely impractical for any real use. Its unpredictable runtime and potential to never terminate make it a perfect example of an inefficient algorithm.",
    best_time: "O(n)",
    average_time: "O(n × n!)",
    worst_time: "∞",
    space: "O(1)",
    applications: "Educational purposes only - demonstrating how not to design algorithms",
};

#[component]
pub fn AlgorithmInfoPanel(algorithm_info: AlgorithmInfo) -> impl IntoView {
    view! {
        <div class="w-full max-w-7xl mx-auto p-4 mt-4">
            <div class="bg-white rounded-lg shadow-md p-6">
                <h2 class="text-2xl font-bold mb-4">{algorithm_info.name}</h2>
                
                <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                    // Theory section
                    <div class="md:col-span-2">
                        <h3 class="text-lg font-semibold mb-2">"Description"</h3>
                        <p class="text-gray-700 mb-4">{algorithm_info.description}</p>
                        <div class="mt-4">
                            <h4 class="font-semibold mb-2">"Common Applications:"</h4>
                            <p class="text-gray-700">{algorithm_info.applications}</p>
                        </div>
                    </div>
                    
                    // Complexity section
                    <div class="bg-gray-50 p-4 rounded-lg">
                        <h3 class="text-lg font-semibold mb-4">"Complexity Analysis"</h3>
                        
                        <div class="mb-4">
                            <h4 class="font-semibold mb-2">"Time Complexity"</h4>
                            <ul class="space-y-2">
                                <li>"Best Case: " {algorithm_info.best_time}</li>
                                <li>"Average Case: " {algorithm_info.average_time}</li>
                                <li>"Worst Case: " {algorithm_info.worst_time}</li>
                            </ul>
                        </div>
                        
                        <div>
                            <h4 class="font-semibold mb-2">"Space Complexity"</h4>
                            <p>{algorithm_info.space}</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}