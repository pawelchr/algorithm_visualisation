use leptos::*;
use std::collections::{VecDeque, BinaryHeap};
use std::cmp::Ordering;
use futures::StreamExt;
use gloo_timers::future::TimeoutFuture;

// Constants
const ROWS: usize = 20;
const COLS: usize = 50;

// Node types
#[derive(Clone, PartialEq)]
enum NodeType {
    Start,
    End,
    Wall,
    Empty,
    Visited,
    Path,
}

// Node structure
#[derive(Clone)]
struct Node {
    row: usize,
    col: usize,
    node_type: NodeType,
}

// Pathfinding algorithms
#[derive(Clone)]
enum Algorithm {
    Dijkstra,
    AStar,
    BFS,
    DFS,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Main component
#[component]
pub fn PathfindingVisualizer() -> impl IntoView {
    let (grid, set_grid) = create_signal(vec![vec![Node { row: 0, col: 0, node_type: NodeType::Empty }; COLS]; ROWS]);
    let (start_node, set_start_node) = create_signal(None::<(usize, usize)>);
    let (end_node, set_end_node) = create_signal(None::<(usize, usize)>);
    let (is_mouse_pressed, set_is_mouse_pressed) = create_signal(false);
    let (selected_algorithm, set_selected_algorithm) = create_signal(Algorithm::Dijkstra);
    let (current_mode, set_current_mode) = create_signal(NodeType::Empty);
    let (is_animating, set_is_animating) = create_signal(false);

    // Initialize the grid
    create_effect(move |_| {
        let mut new_grid = vec![vec![Node { row: 0, col: 0, node_type: NodeType::Empty }; COLS]; ROWS];
        for i in 0..ROWS {
            for j in 0..COLS {
                new_grid[i][j] = Node {
                    row: i,
                    col: j,
                    node_type: NodeType::Empty,
                };
            }
        }
        set_grid(new_grid);
    });

    // Handle mouse events
    let handle_mouse_down = move |row: usize, col: usize| {
        set_is_mouse_pressed(true);
        update_node(row, col, &grid, &set_grid, &start_node, &end_node, &set_start_node, &set_end_node, &current_mode);
    };

    let handle_mouse_enter = move |row: usize, col: usize| {
        if is_mouse_pressed.get() {
            update_node(row, col, &grid, &set_grid, &start_node, &end_node, &set_start_node, &set_end_node, &current_mode);
        }
    };

    let handle_mouse_up = move |_| {
        set_is_mouse_pressed(false);
    };

    // Render grid
    let render_grid = move || {
        grid.get().iter().enumerate().map(|(i, row)| {
            view! {
                <div class="grid-row" style="display: flex;">
                    {row.iter().enumerate().map(|(j, node)| {
                        let node_class = match node.node_type {
                            NodeType::Start => "node-start",
                            NodeType::End => "node-end",
                            NodeType::Wall => "node-wall",
                            NodeType::Empty => "node-empty",
                            NodeType::Visited => "node-visited",
                            NodeType::Path => "node-path",
                        };
                        let background_color = match node.node_type {
                            NodeType::Start => "green",
                            NodeType::End => "red",
                            NodeType::Wall => "black",
                            NodeType::Visited => "lightblue",
                            NodeType::Path => "yellow",
                            NodeType::Empty => "white",
                        };
                        view! {
                            <div
                                class=format!("node {}", node_class)
                                style=format!("width: 20px; height: 20px; border: 1px solid #ccc; background-color: {};", background_color)
                                on:mousedown=move |_| handle_mouse_down(i, j)
                                on:mouseenter=move |_| handle_mouse_enter(i, j)
                                on:mouseup=handle_mouse_up
                            ></div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            }
        }).collect::<Vec<_>>()
    };

    let visualize_pathfinding = move |_| {
        if !is_animating.get() {
            set_is_animating(true);
            if let (Some(start), Some(end)) = (start_node.get(), end_node.get()) {
                let algorithm = selected_algorithm.get();
                let current_grid = grid.get();
                
                let (visited_nodes, path) = match algorithm {
                    Algorithm::Dijkstra => dijkstra(&current_grid, start, end),
                    Algorithm::AStar => astar(&current_grid, start, end),
                    Algorithm::BFS => bfs(&current_grid, start, end),
                    Algorithm::DFS => dfs(&current_grid, start, end),
                };

                spawn_local(async move {
                    animate_path_finding(grid, set_grid, visited_nodes, path, set_is_animating).await;
                });
            } else {
                set_is_animating(false);
            }
        }
    };

    let clear_grid = move |_| {
        initialize_grid(&set_grid);
        set_start_node(None);
        set_end_node(None);
        set_is_animating(false);
    };

    view! {
        <div class="pathfinding-visualizer" style="display: flex; flex-direction: column; align-items: center;">
            <h1>"Pathfinding Visualizer"</h1>
            <div class="controls" style="margin-bottom: 20px;">
                <select on:change=move |ev| {
                    set_selected_algorithm(match event_target_value(&ev).as_str() {
                        "dijkstra" => Algorithm::Dijkstra,
                        "astar" => Algorithm::AStar,
                        "bfs" => Algorithm::BFS,
                        "dfs" => Algorithm::DFS,
                        _ => Algorithm::Dijkstra,
                    });
                }>
                    <option value="dijkstra">"Dijkstra's Algorithm"</option>
                    <option value="astar">"A* Search"</option>
                    <option value="bfs">"Breadth-First Search"</option>
                    <option value="dfs">"Depth-First Search"</option>
                </select>
                <button on:click=visualize_pathfinding disabled=is_animating>"Visualize Pathfinding"</button>
            </div>
            <div class="node-selection" style="margin-bottom: 10px;">
                <button on:click=move |_| set_current_mode(NodeType::Start)>"Select Start"</button>
                <button on:click=move |_| set_current_mode(NodeType::End)>"Select End"</button>
                <button on:click=move |_| set_current_mode(NodeType::Wall)>"Draw Walls"</button>
                <button on:click=clear_grid>"Clear Grid"</button>
            </div>
            <div class="grid" style="display: inline-block; border: 1px solid #000;">
                {render_grid}
            </div>
            <p>
                "Start: " {move || start_node.get().map(|(r, c)| format!("({}, {})", r, c)).unwrap_or_else(|| "Not set".to_string())}
            </p>
            <p>
                "End: " {move || end_node.get().map(|(r, c)| format!("({}, {})", r, c)).unwrap_or_else(|| "Not set".to_string())}
            </p>
        </div>
    }
}

async fn animate_path_finding(
    grid: ReadSignal<Vec<Vec<Node>>>,
    set_grid: WriteSignal<Vec<Vec<Node>>>,
    visited_nodes: Vec<(usize, usize)>,
    path: Vec<(usize, usize)>,
    set_is_animating: WriteSignal<bool>,
) {
    let animation_speed = 10; // milliseconds

    for &(row, col) in &visited_nodes {
        set_grid.update(|g| g[row][col].node_type = NodeType::Visited);
        TimeoutFuture::new(animation_speed).await;
    }

    for &(row, col) in &path {
        set_grid.update(|g| g[row][col].node_type = NodeType::Path);
        TimeoutFuture::new(animation_speed).await;
    }

    set_is_animating(false);
}

// Helper function to update node type
fn update_node(
    row: usize,
    col: usize,
    grid: &ReadSignal<Vec<Vec<Node>>>,
    set_grid: &WriteSignal<Vec<Vec<Node>>>,
    start_node: &ReadSignal<Option<(usize, usize)>>,
    end_node: &ReadSignal<Option<(usize, usize)>>,
    set_start_node: &WriteSignal<Option<(usize, usize)>>,
    set_end_node: &WriteSignal<Option<(usize, usize)>>,
    current_mode: &ReadSignal<NodeType>,
) {
    let current_node_type = grid.get()[row][col].node_type.clone();

    match current_mode.get() {
        NodeType::Start => {
            if let Some(old_start) = start_node.get() {
                set_grid.update(|g| g[old_start.0][old_start.1].node_type = NodeType::Empty);
            }
            set_grid.update(|g| g[row][col].node_type = NodeType::Start);
            set_start_node(Some((row, col)));
        }
        NodeType::End => {
            if let Some(old_end) = end_node.get() {
                set_grid.update(|g| g[old_end.0][old_end.1].node_type = NodeType::Empty);
            }
            set_grid.update(|g| g[row][col].node_type = NodeType::End);
            set_end_node(Some((row, col)));
        }
        NodeType::Wall => {
            if current_node_type != NodeType::Start && current_node_type != NodeType::End {
                set_grid.update(|g| {
                    g[row][col].node_type = if current_node_type == NodeType::Wall {
                        NodeType::Empty
                    } else {
                        NodeType::Wall
                    };
                });
            }
        }
        _ => {}
    }
}

fn dijkstra(grid: &Vec<Vec<Node>>, start: (usize, usize), end: (usize, usize)) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut visited_nodes = Vec::new();
    let mut distances = vec![vec![usize::MAX; COLS]; ROWS];
    let mut prev = vec![vec![None; COLS]; ROWS];
    let mut heap = BinaryHeap::new();

    distances[start.0][start.1] = 0;
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        let (row, col) = position;
        
        if position == end {
            break;
        }

        if cost > distances[row][col] {
            continue;
        }

        visited_nodes.push(position);

        for (dr, dc) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;

            if new_row >= 0 && new_row < ROWS as i32 && new_col >= 0 && new_col < COLS as i32 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                if grid[new_row][new_col].node_type != NodeType::Wall {
                    let new_cost = cost + 1;
                    if new_cost < distances[new_row][new_col] {
                        distances[new_row][new_col] = new_cost;
                        prev[new_row][new_col] = Some((row, col));
                        heap.push(State { cost: new_cost, position: (new_row, new_col) });
                    }
                }
            }
        }
    }

    let path = reconstruct_path(&prev, end);
    (visited_nodes, path)
}

fn astar(grid: &Vec<Vec<Node>>, start: (usize, usize), end: (usize, usize)) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut visited_nodes = Vec::new();
    let mut g_score = vec![vec![usize::MAX; COLS]; ROWS];
    let mut f_score = vec![vec![usize::MAX; COLS]; ROWS];
    let mut prev = vec![vec![None; COLS]; ROWS];
    let mut heap = BinaryHeap::new();

    g_score[start.0][start.1] = 0;
    f_score[start.0][start.1] = heuristic(start, end);
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost: _, position }) = heap.pop() {
        let (row, col) = position;
        
        if position == end {
            break;
        }

        visited_nodes.push(position);

        for (dr, dc) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;

            if new_row >= 0 && new_row < ROWS as i32 && new_col >= 0 && new_col < COLS as i32 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                if grid[new_row][new_col].node_type != NodeType::Wall {
                    let tentative_g_score = g_score[row][col] + 1;
                    if tentative_g_score < g_score[new_row][new_col] {
                        prev[new_row][new_col] = Some((row, col));
                        g_score[new_row][new_col] = tentative_g_score;
                        f_score[new_row][new_col] = g_score[new_row][new_col] + heuristic((new_row, new_col), end);
                        heap.push(State { cost: f_score[new_row][new_col], position: (new_row, new_col) });
                    }
                }
            }
        }
    }

    let path = reconstruct_path(&prev, end);
    (visited_nodes, path)
}

fn bfs(grid: &Vec<Vec<Node>>, start: (usize, usize), end: (usize, usize)) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut visited_nodes = Vec::new();
    let mut visited = vec![vec![false; COLS]; ROWS];
    let mut prev = vec![vec![None; COLS]; ROWS];
    let mut queue = VecDeque::new();

    visited[start.0][start.1] = true;
    queue.push_back(start);

    while let Some(position) = queue.pop_front() {
        let (row, col) = position;
        
        visited_nodes.push(position);

        if position == end {
            break;
        }

        for (dr, dc) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;

            if new_row >= 0 && new_row < ROWS as i32 && new_col >= 0 && new_col < COLS as i32 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                if !visited[new_row][new_col] && grid[new_row][new_col].node_type != NodeType::Wall {
                    visited[new_row][new_col] = true;
                    prev[new_row][new_col] = Some((row, col));
                    queue.push_back((new_row, new_col));
                }
            }
        }
    }

    let path = reconstruct_path(&prev, end);
    (visited_nodes, path)
}

fn dfs(grid: &Vec<Vec<Node>>, start: (usize, usize), end: (usize, usize)) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut visited_nodes = Vec::new();
    let mut visited = vec![vec![false; COLS]; ROWS];
    let mut prev = vec![vec![None; COLS]; ROWS];
    let mut stack = Vec::new();

    stack.push(start);

    while let Some(position) = stack.pop() {
        let (row, col) = position;
        
        if !visited[row][col] {
            visited[row][col] = true;
            visited_nodes.push(position);

            if position == end {
                break;
            }

            for (dr, dc) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let new_row = row as i32 + dr;
                let new_col = col as i32 + dc;

                if new_row >= 0 && new_row < ROWS as i32 && new_col >= 0 && new_col < COLS as i32 {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;

                    if !visited[new_row][new_col] && grid[new_row][new_col].node_type != NodeType::Wall {
                        prev[new_row][new_col] = Some((row, col));
                        stack.push((new_row, new_col));
                    }
                }
            }
        }
    }

    let path = reconstruct_path(&prev, end);
    (visited_nodes, path)
}

fn heuristic(a: (usize, usize), b: (usize, usize)) -> usize {
    ((a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()) as usize
}

fn reconstruct_path(prev: &Vec<Vec<Option<(usize, usize)>>>, end: (usize, usize)) -> Vec<(usize, usize)> {
    let mut path = Vec::new();
    let mut current = end;
    while let Some(position) = prev[current.0][current.1] {
        path.push(current);
        current = position;
    }
    path.push(current);
    path.reverse();
    path
}

fn initialize_grid(set_grid: &WriteSignal<Vec<Vec<Node>>>) {
    let mut new_grid = vec![vec![Node { row: 0, col: 0, node_type: NodeType::Empty }; COLS]; ROWS];
    for i in 0..ROWS {
        for j in 0..COLS {
            new_grid[i][j] = Node {
                row: i,
                col: j,
                node_type: NodeType::Empty,
            };
        }
    }
    set_grid(new_grid);
}