use crate::sorting::BarColor;
use leptos::*;
use leptos_charts::{BarChart, Color, BarChartOptions, Palette};


#[component]
pub fn SortingChart(
    steps: ReadSignal<Vec<Vec<f64>>>,
    palettes: ReadSignal<Vec<Vec<BarColor>>>,
) -> impl IntoView {
    let current_step = create_rw_signal(0);
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
        <div>
            <div class="flex justify-center mt-1">
                <BarChart
                    values=Signal::derive(move || step()).into()
                    pallete=Signal::derive(move || Palette(palette())).into()
                    options=options
                    attr:style="margin-top:5px "
                    attr:preserveAspectRatio="none"
                    attr:width="500"
                    attr:height="500"
                />
            </div>
            <div class="flex items-center justify-center">
                // <button on:click=move |_| {
                // set_current_step.update(|n| *n -= 1);
                // }>"Previous step"</button>
                // <button
                // prop:disabled=move || { if disable() { true } else { false } }
                // on:click=move |_| {
                // if current_step() < steps().len() - 1 {
                // set_current_step.update(|n| *n += 1);
                // } else {
                // set_disable(true)
                // }
                // }
                // >
                // "Next step"
                // </button>
                <button
                    on:click=move |_| {
                        current_step
                            .update(|n| {
                                if *n > 0 {
                                    *n -= 1;
                                }
                            });
                    }
                    disabled=move || current_step.get() == 0
                >
                    "Previous step"
                </button>
                <button
                    on:click=move |_| {
                        current_step
                            .update(|n| {
                                if *n < steps().len().saturating_sub(1) {
                                    *n += 1;
                                }
                            });
                    }
                    disabled=move || {
                        let max_step = steps().len().saturating_sub(1);
                        current_step.get() >= max_step
                    }
                >
                    "Next step"
                </button>
                // <p>"step: "{move || step().to_string()}</p>
                <p>"current step: "{move || current_step().to_string()}</p>
                <p>"steps.len: "{move || steps().len().to_string()}</p>
            </div>
        </div>
    }
}

// #[component]
// pub fn SortingChartistry(
//     debug: Signal<bool>,
//     data: Signal<SortRes>,
// ) -> impl IntoView {
//     let grey_color = Colour::from_rgb(108, 108, 108);
//     let green_color = Colour::from_rgb(0, 255, 0);
//     let orange_color = Colour::from_rgb(227, 150, 62);

//     let is_data_non_empty = Signal::derive(move || data().steps.data.len() != 0);
//     // let data = Signal::derive(data);
//     // // if is_data_non_empty.get() {
//     // let step = data().steps.return_index(0).clone();
//     // let first_colour = match data().steps.data[0][0].colour {
//     //     BarColor::Grey => grey_color.clone(),
//     //     BarColor::Green => green_color.clone(),
//     //     _ => orange_color.clone(),
//     // };

//     // let scheme = ColourScheme::new(first_colour, vec![Colour::from_rgb(1, 1, 1)]);
//     // let data_steps = create_rw_signal(data().steps.data[0].clone());
//     // let step_clone = step.clone();
//     // let series = Series::new(move |data: &MyData| step_clone[0].x.clone())
//     //     .bar(move |data: &MyData| step[0].y1.clone()).with_colours(scheme);
// //     view! {
// //         <Chart
// //             aspect_ratio=AspectRatio::from_outer_height(300.0, 1.2)
// //             debug=debug
// //             series=series
// //             data=data_steps

// //             left=TickLabels::aligned_floats()
// //             inner=[
// //                 AxisMarker::left_edge().into_inner(),
// //                 AxisMarker::bottom_edge().into_inner(),
// //                 YGridLine::default().into_inner(),
// //             ]
// //         />
// //         // <div class="fixed bottom-0 items-center">
// //         //     <button on:click=move |_| {
// //         //         set_current_step.update(|n| *n -= 1);
// //         //     }>"Previous step"</button>
// //         //     <button
// //         //         prop:disabled=move || { if disable() { true } else { false } }
// //         //         on:click=move |_| {
// //         //             if current_step() < steps().len() - 1 {
// //         //                 set_current_step.update(|n| *n += 1);
// //         //             } else {
// //         //                 set_disable(true)
// //         //             }
// //         //         }
// //         //     >
// //         //         "Next step"
// //         //     </button>
// //         //     // <p>"step: "{move || step().to_string()}</p>
// //         //     // <p>"current step: "{move || current_step().to_string()}</p>
// //         //     // <p>"steps.len: "{move || steps().len().to_string()}</p>
// //         // </div>
// //     }
// // }
// view! {
//     <div>
//         {move || {
//             if is_data_non_empty.get() {
//                 let step = data().steps.return_index(0).clone();
//                 let first_colour = match data().steps.data[0][0].colour {
//                     BarColor::Grey => grey_color.clone(),
//                     BarColor::Green => green_color.clone(),
//                     _ => orange_color.clone(),
//                 };
//                 let scheme = ColourScheme::new(first_colour, vec![Colour::from_rgb(1, 1, 1)]);
//                 let data_steps = create_rw_signal(data().steps.data[0].clone());
//                 let step_clone = step.clone();
//                 let series = Series::new(move |data: &MyData| step_clone[0].x.clone())
//                     .bar(move |data: &MyData| step[0].y1.clone())
//                     .with_colours(scheme);
//                 view! {
//                     <Chart
//                         aspect_ratio=AspectRatio::from_outer_height(500.0, 1.2)
//                         debug=debug
//                         series=series
//                         data=data_steps

//                         left=TickLabels::aligned_floats()
//                         inner=[
//                             AxisMarker::left_edge().into_inner(),
//                             AxisMarker::bottom_edge().into_inner(),
//                             YGridLine::default().into_inner(),
//                         ]
//                     />
//                 }
//             } else {
//                 view! {
//                     <p>{move || format!("is not emtpy: {:?}", is_data_non_empty.get())}</p>
//                     <p>"Enter numbers"</p>
//                     <p>{move || format!("data {:?}", data.get())}</p>
//                 }
//                     .into_view()
//             }
//         }}
//     </div>
// }
// }

// #[component]
// pub fn SortingChartistry(
//     debug: Signal<bool>,
//     data: Signal<SortRes>,
// ) -> impl IntoView {
//     let current_step = create_rw_signal(0);

//     // Reset current_step when data changes
//     create_effect(move |_| {
//         data.track();
//         current_step.set(0);
//     });

//     let current_data = create_memo(move |_| {
//         data.with(|d| {
//             d.steps.data.get(current_step.get())
//                 .cloned()
//                 .unwrap_or_default()
//         })
//     });

//     // let bars = create_rw_signal(move |_| {
//     //     current_data.with(|data| {
//     //         data.iter()
//     //             .enumerate()
//     //             .map(|(index, step)| {
//     //                 let y1 = step.y1;
//     //                 let color = match step.colour {
//     //                     BarColor::Orange => Colour::from_rgb(255, 127, 80),
//     //                     BarColor::Green => Colour::from_rgb(50, 205, 50),
//     //                     BarColor::Grey => Colour::from_rgb(192, 192, 192),
//     //                 };
//     //                 Bar::new(move |_: &MyData| y1).with_colour(color)
//     //             })
//     //             .collect::<Vec<_>>().iter()
//     //     })
//     // });
//     let x = 1.0;
//     let bars_data = Signal::derive(move || {current_data.with(|data| {
//         data.iter()
//             .enumerate()
//             .map(|(index,  step)| {
//                 let y1 = step.y1;

//                 Bar::new(move |_: &MyData| y1).with_colour(step.colour).with_name(index.to_string())
//             })
//             .collect::<Vec<_>>()
//     })});

//     // let series = Signal::derive(move || {Series::new(|step: &MyData | step.x)
//     //     .with_min_y(0.0)
//     //     .bar(|current_data: &MyData| current_data.y1).with_colours(color_scheme())});
//     let color_scheme = Signal::derive(move || {
//         current_data.with(|data| {
//             data.iter()
//             .enumerate()
//             .map(|(index, step)| {
//                 step.colour
//             })
//             .collect::<Vec<Colour>>()
//         })
//     });

//     let series = Signal::derive(move || {Series::new(|step: &MyData | step.x)
//         .with_min_y(0.0)
//         .bar(Bar::new(|step: &MyData| step.y1)).with_colours(ColourScheme::new(color_scheme()[0], color_scheme()[1..].to_vec()))});


//     // let series = Series::new(|step: &MyData | step.x)
//     //     .with_min_y(0.0).bar(|step: &MyData| step.y1).with_colours(color_scheme());



//     view! {
//         <div>
//         { move ||
//             {
//                 if !data.with(|d| d.steps.data.is_empty()) {
//                     view! {
//                         <div class="flex justify-center mt-2">
//                         <Chart
//                             aspect_ratio=AspectRatio::from_outer_height(500.0, 1.0)
//                             debug=debug
//                             series=series()
//                             data=current_data
//                             left=TickLabels::aligned_floats()
//                             bottom=TickLabels::aligned_floats()
//                             inner=[
//                                 AxisMarker::left_edge().into_inner(),
//                                 AxisMarker::bottom_edge().into_inner(),
//                                 YGridLine::default().into_inner(),
//                             ]
//                             tooltip=Tooltip::left_cursor()
//                         />
//                         </div>
//                         <p>"Current step: " {current_step}</p>
//                         <p>"Total steps: " {move || data.with(|d| d.steps.data.len())}</p>
//                         <p>"Current data: " {move || format!("{:?}",current_data())}</p>
//                         <p>"ColorScheme: " {move || format!("{:?}", color_scheme())}</p>
//                     }
//                 } else {
//                     view! { <p>"No data available. Please submit some data."</p>
//                 <p></p> }
//                 }
//             }
//         }
//             <div class="items-center">
//                 <button
//                     on:click=move |_| {
//                         current_step.update(|n| {
//                             if *n > 0 { *n -= 1 }
//                         });
//                     }
//                     disabled=move || current_step.get() == 0
//                 >
//                     "Previous step"
//                 </button>
//                 <button
//                     on:click=move |_| {
//                         current_step.update(|n| {
//                             if *n < data.with(|d| d.steps.data.len().saturating_sub(1)) {
//                                 *n += 1
//                             }
//                         });
//                     }
//                     disabled=move || {
//                         let max_step = data.with(|d| d.steps.data.len().saturating_sub(1));
//                         current_step.get() >= max_step
//                     }
//                 >
//                     "Next step"
//                 </button>
//             </div>
//         </div>
//     }
// }