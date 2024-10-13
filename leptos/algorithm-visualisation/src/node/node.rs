use leptos::*;
use stylers::style;

#[component]
pub fn Node(
    row: usize,
    col: usize,
    is_start: Signal<bool>, 
    is_finish: Signal<bool>, 
    is_wall: Signal<bool>,
    is_visited: Signal<bool>,
    is_shortest: Signal<bool>,
    on_mouse_enter: Box<dyn Fn(usize, usize)>,
    on_mouse_down: Box<dyn Fn(usize, usize)>,
    on_mouse_up: Box<dyn Fn()>,
    width: f64,
    height: f64,
    num_rows: usize,
    num_columns: usize,
) -> impl IntoView {
    let styler_class = style! { "Node",
        .node {
            width: var(--width);
            height: var(--height);
            border: 1px solid rgb(175, 216, 248);
            /*   outline: 0.2px solid rgb(175, 216, 248); */
            display: inline-block;
          }
          
          .node-start {
            background-color: green;
          }
          
          .node-finish {
            background-color: red;
          }
          
          .node-finish-reached {
            animation-name: finishVisitedAnimation;
            animation-duration: 1.5s;
            animation-timing-function: ease-out;
            animation-direction: alternate;
            animation-fill-mode: forwards;
          }
          
          .node-wall {
            width: var(--width);
            height: var(--height);
            background-color: rgb(2, 36, 51);
            border: 1px solid rgb(2, 36, 51);
            display: inline-block;
          }
          
          .node-wall-animated {
            width: var(--width);
            height: var(--height);
            display: inline-block;
            animation-name: wallAnimation;
            animation-duration: 0.5s;
            animation-timing-function: ease-out;
            animation-direction: alternate;
            animation-fill-mode: forwards;
          }
          
          .node-visited {
            animation-name: visitedAnimation;
            animation-duration: 1.5s;
            animation-timing-function: ease-out;
            animation-direction: alternate;
            animation-fill-mode: forwards;
          }
          
          .node-shortest-path {
            animation-name: shortestPathAnimation;
            animation-duration: 0.5s;
            animation-timing-function: ease-out;
            animation-direction: alternate;
            animation-fill-mode: forwards;
          }

          @keyframes wallAnimation {
            0% {
              transform: scale(0.4);
              background-color: rgb(2, 36, 51);
              border-radius: 30%;
            }
          
            50% {
              transform: scale(0.6);
              background-color: rgb(2, 36, 51);
              border-radius: 20%;
            }
          
            75% {
              transform: scale(0.8);
              background-color: rgb(2, 36, 51);
              border-radius: 10%;
            }
          
            100% {
              transform: scale(1);
              background-color: rgb(2, 36, 51);
              border: 1px solid rgb(2, 36, 51);
            }
          }
          
          @keyframes visitedAnimation {
            0% {
              transform: scale(0.3);
              background-color: rgb(255, 0, 255);
              border-radius: 100%;
            }
          
            50% {
              transform: scale(0.5);
              background-color: rgba(10, 55, 95, 0.75);
              border-radius: 75%;
            }
          
            75% {
              transform: scale(0.7);
              background-color: rgba(20, 110, 140, 0.75);
              border-radius: 50%;
            }
          
            100% {
              transform: scale(1);
              background-color: rgba(30, 165, 185, 0.75);
            }
          }
          
          @keyframes shortestPathAnimation {
            0% {
              transform: scale(0.3);
              background-color: rgba(255, 0, 255, 0.75);
              border-radius: 100%;
            }
          
            50% {
              transform: scale(0.5);
              background-color: rgba(255, 0, 255, 0.75);
              border-radius: 75%;
            }
          
            75% {
              transform: scale(0.7);
              background-color: rgba(255, 0, 255, 0.75);
              border-radius: 50%;
            }
          
            100% {
              transform: scale(1);
              background-color: rgba(255, 0, 255, 0.75);
            }
          }
          
          @keyframes finishVisitedAnimation {
            0% {
              transform: scale(0.5);
              background-color: red;
              border-radius: 50%;
            }
          
            50% {
              transform: scale(0.8);
              background-color: red;
              border-radius: 25%;
            }
          
            75% {
              transform: scale(1.2);
              background-color: red;
              border-radius: 20%;
            }
          
            100% {
              transform: scale(1);
              background-color: red;
            }
          }
    };
    let cell_width = (width - 15.0).floor() / num_columns as f64;
    let cell_height = if width > 1500.0 {
        (height - 70.0).floor() / num_rows as f64
    } else if width > 1000.0 {
        (height - 70.0).floor() / num_rows as f64
    } else if width > 500.0 {
        (height - 60.0).floor() / num_rows as f64
    } else {
        (height - 50.0).floor() / num_rows as f64
    };
    let class_name = Signal::derive(move ||{
        if is_start.get() {
            "node node-start".to_string()
        } else if is_finish.get() {
            "node node-finish".to_string()
        } else if is_wall.get() {
            "node-wall".to_string()
        } else if is_shortest.get() {
            "node node-shortest-path".to_string()
        } else if is_visited.get() {
            "node node-visited".to_string()
        } else {
            "node".to_string()
        }
    });

    view! {
        <div
        class=move || {
            let mut classes = vec!["node"];
            if is_start.get() {
                classes.push("node-start");
            }
            if is_finish.get() {
                classes.push("node-finish");
            }
            if is_wall.get() {
                classes.push("node-wall");
            }
            if is_shortest.get() {
                classes.push("node-shortest-path");
            }
            if is_visited.get() {
                classes.push("node-visited");
            }
            classes.join(" ")
        }
            id=format!("node-{}-{}", row, col)
            style=format!("--width: {}px; --height: {}px;", cell_width, cell_height)
            on:mouseenter=move |_| on_mouse_enter(row, col)
            on:mousedown=move |_| on_mouse_down(row, col)
            on:mouseup=move |_| on_mouse_up()
        ></div>
    }
}

#[component]
pub fn ShowNode() -> impl IntoView {
    let styler_class = style! { "Node",
        .node {
            width: var(--width);
            height: var(--height);
            border: 1px solid rgb(175, 216, 248);
            /*   outline: 0.2px solid rgb(175, 216, 248); */
            display: inline-block;
          }
          
          .node-start {
            background-color: green;
          }
          
          .node-finish {
            background-color: red;
          }
          
          .node-finish-reached {
            animation-name: finishVisitedAnimation;
            animation-duration: 1.5s;
            animation-timing-function: ease-out;
            animation-direction: alternate;
            animation-fill-mode: forwards;
          }
          
          .node-wall {
            width: var(--width);
            height: var(--height);
            background-color: rgb(2, 36, 51);
            border: 1px solid rgb(2, 36, 51);
            display: inline-block;
          }
          
          .node-wall-animated {
            width: var(--width);
            height: var(--height);
            display: inline-block;
            animation-name: wallAnimation;
            animation-duration: 0.5s;
            animation-timing-function: ease-out;
            animation-direction: alternate;
            animation-fill-mode: forwards;
          }
          
          .node-visited {
            animation-name: visitedAnimation;
            animation-duration: 1.5s;
            animation-timing-function: ease-out;
            animation-direction: alternate;
            animation-fill-mode: forwards;
          }
          
          .node-shortest-path {
            animation-name: shortestPathAnimation;
            animation-duration: 0.5s;
            animation-timing-function: ease-out;
            animation-direction: alternate;
            animation-fill-mode: forwards;
          }

          @keyframes wallAnimation {
            0% {
              transform: scale(0.4);
              background-color: rgb(2, 36, 51);
              border-radius: 30%;
            }
          
            50% {
              transform: scale(0.6);
              background-color: rgb(2, 36, 51);
              border-radius: 20%;
            }
          
            75% {
              transform: scale(0.8);
              background-color: rgb(2, 36, 51);
              border-radius: 10%;
            }
          
            100% {
              transform: scale(1);
              background-color: rgb(2, 36, 51);
              border: 1px solid rgb(2, 36, 51);
            }
          }
          
          @keyframes visitedAnimation {
            0% {
              transform: scale(0.3);
              background-color: rgb(255, 0, 255);
              border-radius: 100%;
            }
          
            50% {
              transform: scale(0.5);
              background-color: rgba(10, 55, 95, 0.75);
              border-radius: 75%;
            }
          
            75% {
              transform: scale(0.7);
              background-color: rgba(20, 110, 140, 0.75);
              border-radius: 50%;
            }
          
            100% {
              transform: scale(1);
              background-color: rgba(30, 165, 185, 0.75);
            }
          }
          
          @keyframes shortestPathAnimation {
            0% {
              transform: scale(0.3);
              background-color: rgba(255, 0, 255, 0.75);
              border-radius: 100%;
            }
          
            50% {
              transform: scale(0.5);
              background-color: rgba(255, 0, 255, 0.75);
              border-radius: 75%;
            }
          
            75% {
              transform: scale(0.7);
              background-color: rgba(255, 0, 255, 0.75);
              border-radius: 50%;
            }
          
            100% {
              transform: scale(1);
              background-color: rgba(255, 0, 255, 0.75);
            }
          }
          
          @keyframes finishVisitedAnimation {
            0% {
              transform: scale(0.5);
              background-color: red;
              border-radius: 50%;
            }
          
            50% {
              transform: scale(0.8);
              background-color: red;
              border-radius: 25%;
            }
          
            75% {
              transform: scale(1.2);
              background-color: red;
              border-radius: 20%;
            }
          
            100% {
              transform: scale(1);
              background-color: red;
            }
          }
    };
    let (is_start, set_start) = create_signal(false);
    let (is_finish, set_finish) = create_signal(false);
    let (is_wall, set_wall) = create_signal(false);
    let (is_visited, set_visited) = create_signal(false);
    let (is_shortest, set_shortest) = create_signal(false);

    // Example dimensions and grid size
    let width = 800.0;
    let height = 600.0;
    let num_rows = 10;
    let num_columns = 10;

    // Create mock callback functions
    let on_mouse_enter = Box::new(move |row: usize, col: usize| {
        log::info!("Mouse entered node at: ({}, {})", row, col);
    });

    let on_mouse_down = Box::new(move |row: usize, col: usize| {
        log::info!("Mouse down on node at: ({}, {})", row, col);
        // You can set some state here as needed
    });

    let on_mouse_up = Box::new(move || {
        log::info!("Mouse up");
        // You can reset some state here if necessary
    });

    // Create an example Node instance
    let row = 0; // Example row
    let col = 0; // Example column

    view! { class=styler_class,
        <div>
            <Node 
                row={row} 
                col={col} 
                is_start={is_start.into()} 
                is_finish={is_finish.into()} 
                is_wall={is_wall.into()} 
                is_visited={is_visited.into()} 
                is_shortest={is_shortest.into()} 
                on_mouse_enter={on_mouse_enter} 
                on_mouse_down={on_mouse_down} 
                on_mouse_up={on_mouse_up} 
                width={width} 
                height={height} 
                num_rows={num_rows} 
                num_columns={num_columns} 
            />
            // You can render more Nodes or other UI elements here
        </div>
    }
}