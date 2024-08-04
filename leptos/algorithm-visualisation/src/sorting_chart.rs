use crate::BarColor;
use leptos::*;
use leptos_charts::*;
use std::f64;

#[component]
pub fn SortingChart(
    steps: ReadSignal<Vec<Vec<f64>>>,
    palettes: ReadSignal<Vec<Vec<BarColor>>>,
) -> impl IntoView {
    let (disable, set_disable) = create_signal(false);
    let (current_step, set_current_step) = create_signal(0);
    let grey_color = Color::RGB(108, 108, 108);
    let green_color = Color::RGB(0, 255, 0);
    let orange_color = Color::RGB(227, 150, 62);

    // Memoize the palettes so it doesn't recompute unnecessarily
    let palettes_memo = create_memo(move |_| palettes());

    // Define the palette for the current step
    let palette = create_memo(move |_| {
        let current_index = current_step();
        let step_palette = &palettes_memo()[current_index];
        step_palette
            .iter()
            .map(|color| match color {
                BarColor::Grey => grey_color.clone(),
                BarColor::Green => green_color.clone(),
                _ => orange_color.clone(),
            })
            .collect::<Vec<Color>>()
    });

    let options = BarChartOptions { max_ticks: 4 };

    // Define the step values for the current step
    let step = create_memo(move |_| {
        let current_index = current_step();
        steps()[current_index].clone()
    });

    view! {
        <BarChart
            values=Signal::derive(move || step()).into()
            pallete=Signal::derive(move || Palette(palette())).into()
            options=options
            attr:style="margin-top:5px"
            attr:preserveAspectRatio="none"
            attr:width="500"
            attr:height="500"
        />
        <button
            on:click=move |_| { set_current_step.update(|n| *n -= 1); }
        >
            "Previous step"
        </button>
        <button
            prop:disabled=move || {if disable() {true} else {false}}
            on:click=move |_| { if current_step() < steps().len() - 1 {
    set_current_step.update(|n| *n += 1); } else {
        set_disable(true)
    }
    }
        >
            "Next step"
        </button>
        // <p>"step: "{move || step().to_string()}</p>
        <p>"current step: "{move || current_step().to_string()}</p>
        <p>"steps.len: "{move || steps().len().to_string()}</p>
    }
}
