use std::f64;

use leptos::*;
use leptos_charts::*;

#[component]
pub fn SortingChart(data: ReadSignal<Vec<Vec<f64>>>) -> impl IntoView {
    let steps: MaybeSignal<Vec<Vec<f64>>> = data.into(); //todo - how to turn readsignal to vec![]
    let palette = vec![
        Color::RGB(108, 108, 108),
        Color::RGB(108, 108, 108),
        Color::RGB(0, 255, 0),
        Color::RGB(108, 108, 108),
        Color::RGB(108, 108, 108),
        Color::RGB(108, 108, 108),
        Color::RGB(0, 255, 0),
    ];
    let options = Box::new(BarChartOptions {
        max_ticks: 4,
        color: Box::new(Palette(palette)),
    });

    view! {
    <BarChart
        values=steps[0].into()
        options=options
        attr:style="margin-top:5px"
        attr:preserveAspectRatio="none"
        attr:width="500"
        attr:height="500"
    />
    }
}
