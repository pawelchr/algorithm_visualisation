use leptos::*;
use leptos_router::{Route, Router, Routes};
use crate::path_finding_menu::PathfindingVisualizer;
use crate::bubblesortviz::BubbleSortVisualizer;
use crate::quicksortviz::QuickSortVisualizer;
use crate::mergesortviz::MergeSortVisualizer;
use crate::insertsortviz::InsertionSortVisualizer;
use crate::selectionsortviz::SelectionSortVisualizer;
use crate::bogosortviz::BogoSortVisualizer;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/bubble-sort" view=BubbleSortVisualizer/>
                <Route path="/quick-sort" view=QuickSortVisualizer/>
                <Route path="/merge-sort" view=MergeSortVisualizer/>
                <Route path="/insert-sort" view=InsertionSortVisualizer/>
                <Route path="/selection-sort" view=SelectionSortVisualizer/>
                <Route path="/bogo-sort" view=BogoSortVisualizer/>
                <Route path="/path-finding" view=PathfindingVisualizer />
            </Routes>
        </Router>
    }
}