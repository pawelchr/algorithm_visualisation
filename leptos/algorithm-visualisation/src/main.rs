mod navbar;
mod sorting;
mod sorting_chart;
mod sorting_menu;
mod path_finding_menu;
mod path_finding;
mod app;
mod bubblesortviz;
mod quicksortviz;
mod mergesortviz;
mod insertsortviz;
mod selectionsortviz;
mod bogosortviz;
mod sorting_controls;
mod sorting_info;
mod landing_page;

use leptos::*;
use app::App;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}
