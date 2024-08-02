use leptos::*;
use leptos_charts::*;

#[component]
pub fn SortingChart(data: ReadSignal<Vec<f64>>) -> impl IntoView {
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
        values=data.into()
        options=options
        attr:style="margin-top:5px"
        attr:preserveAspectRatio="none"
        attr:width="500"
        attr:height="500"
    />
    }
}
