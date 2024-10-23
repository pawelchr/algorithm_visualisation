use leptos::*;
use leptos_router::{Route, Router, Routes};
use crate::sorting_menu::SortingMenu;
use crate::path_finding_menu::PathfindingVisualizer;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/sorting" view=SortingMenu />
                <Route path="/path_finding" view=PathfindingVisualizer />
            </Routes>
        </Router>
    }
}