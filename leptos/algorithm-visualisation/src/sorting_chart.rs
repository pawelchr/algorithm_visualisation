use leptos::*;
use leptos_charts::*;
use std::f64;

#[component]
pub fn SortingChart(data: ReadSignal<Vec<Vec<f64>>>) -> impl IntoView {
    let steps = data();
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
    let (current_step, set_current_step) = create_signal(0);
    let step = move || -> Vec<f64> { steps[current_step()].clone() };

    view! {
    <BarChart
        values=MaybeSignal::from(step())
        options=options
        attr:style="margin-top:5px"
        attr:preserveAspectRatio="none"
        attr:width="500"
        attr:height="500"
    />
    <button
        on:click= move |_| {set_current_step.update(|n| *n -= 1);}
        >
        "Previous step"
    </button>
    <button
            on:click=move |_| {set_current_step.update(|n| *n += 1);}
            >
        "Next step"
    </button>
    <p>{move || step()}
            {move || current_step()}
            </p>
    <p>{}</p>
    }
}
