use leptos::*;
use std::collections::{VecDeque, BinaryHeap, HashSet};
use std::cmp::Ordering;
use gloo_timers::future::TimeoutFuture;
use rand::prelude::SliceRandom;
use rand::Rng;
use crate::navbar::NavBar;

// Constants
const ROWS: usize = 20;
const COLS: usize = 50;

#[derive(Clone, Copy, PartialEq)]
enum DrawingMode {
    None,
    Drawing,
    Erasing,
}

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
    Swarm,
}

#[derive(Clone, PartialEq)]
enum AnimationSpeed {
    Slow,
    Medium,
    Fast,
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
    let (visited_nodes, set_visited_nodes) = create_signal(Vec::new());
    let (path, set_path) = create_signal(Vec::new());
    let (animation_speed, set_animation_speed) = create_signal(AnimationSpeed::Medium);
    let (wall_drawing_mode, set_wall_drawing_mode) = create_signal(DrawingMode::None);

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

    let handle_mouse_down = move |row: usize, col: usize, e: web_sys::MouseEvent| {
        if e.button() == 0 {
            e.prevent_default();
            set_is_mouse_pressed(true);
            
            // Set drawing mode only if we're in wall mode
            if current_mode.get() == NodeType::Wall {
                let mode = if grid.get()[row][col].node_type == NodeType::Wall {
                    DrawingMode::Erasing
                } else {
                    DrawingMode::Drawing
                };
                set_wall_drawing_mode(mode);
                update_node(row, col, &grid, &set_grid, &start_node, &end_node, &set_start_node, &set_end_node, &current_mode, mode);
            } else {
                update_node(row, col, &grid, &set_grid, &start_node, &end_node, &set_start_node, &set_end_node, &current_mode, DrawingMode::None);
            }
        }
    };
    
    let handle_mouse_enter = move |row: usize, col: usize, e: web_sys::MouseEvent| {
        e.prevent_default();
        if is_mouse_pressed.get() && current_mode.get() == NodeType::Wall {
            update_node(row, col, &grid, &set_grid, &start_node, &end_node, &set_start_node, &set_end_node, &current_mode, wall_drawing_mode.get());
        }
    };
    
    let handle_mouse_up = move |e: web_sys::MouseEvent| {
        e.prevent_default();
        set_is_mouse_pressed(false);
        set_wall_drawing_mode(DrawingMode::None);
    };

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
                                style=format!("width: 20px; height: 20px; border: 1px solid #ccc; background-color: {}; user-select: none;", background_color)
                                on:mousedown=move |e| handle_mouse_down(i, j, e)
                                on:mouseenter=move |e| handle_mouse_enter(i, j, e)
                                on:mouseup=handle_mouse_up
                                on:mouseleave=handle_mouse_up
                                on:dragstart=|e| e.prevent_default()
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
                
                set_grid.update(|g| clear_path_and_visited(g));
                
                let current_grid = grid.get();
                
                let (visited, path_result) = match algorithm {
                    Algorithm::Dijkstra => dijkstra(&current_grid, start, end),
                    Algorithm::AStar => astar(&current_grid, start, end),
                    Algorithm::BFS => bfs(&current_grid, start, end),
                    Algorithm::DFS => dfs(&current_grid, start, end),
                    Algorithm::Swarm => swarm(&current_grid, start, end),
                };
    
                set_visited_nodes(visited);
                set_path(path_result);
    
                spawn_local(async move {
                    animate_path_finding(set_grid, visited_nodes, path, set_is_animating, animation_speed).await;
                });
            } else {
                set_is_animating(false);
            }
        }
    };

    let generate_labyrinth = move |_| {
        if let (Some(start), Some(end)) = (start_node.get(), end_node.get()) {
            set_grid.update(|g| {
                for row in g.iter_mut() {
                    for node in row.iter_mut() {
                        node.node_type = NodeType::Wall;
                    }
                }

                generate_maze(g, start, end);

                g[start.0][start.1].node_type = NodeType::Start;
                g[end.0][end.1].node_type = NodeType::End;
            });
        }
    };

    let clear_grid = move |_| {
        initialize_grid(&set_grid);
        set_start_node(None);
        set_end_node(None);
        set_is_animating(false);
    };

    view! {
        <NavBar/>
        <div class="pathfinding-visualizer mt-2" style="display: flex; flex-direction: column; align-items: center;">
            //<h1>"Pathfinding Visualizer"</h1>
            <div class="controls" style="margin-bottom: 20px;">
                <select on:change=move |ev| {
                    set_selected_algorithm(match event_target_value(&ev).as_str() {
                        "dijkstra" => Algorithm::Dijkstra,
                        "astar" => Algorithm::AStar,
                        "bfs" => Algorithm::BFS,
                        "dfs" => Algorithm::DFS,
                        "swarm" => Algorithm::Swarm,
                        _ => Algorithm::Dijkstra,
                    });
                }>
                    <option value="dijkstra">"Dijkstra's Algorithm"</option>
                    <option value="astar">"A* Search"</option>
                    <option value="bfs">"Breadth-First Search"</option>
                    <option value="dfs">"Depth-First Search"</option>
                    <option value="swarm">"Swarm Algorithm"</option>
                </select>
                <button
                class="ml-1 px-4 py-2 bg-blue-500 text-white rounded"
                on:click=visualize_pathfinding disabled=is_animating>"Visualize Pathfinding"</button>
            </div>

            <div class="grid" style="display: inline-block; border: 1px solid #000;">
                {render_grid}
            </div>
            <div class="node-selection" style="margin-bottom: 10px;">
                <button
                class="m-1 px-4 py-2 bg-blue-500 text-white rounded"
                on:click=move |_| set_current_mode(NodeType::Start)>"Select Start"</button>
                <button
                class="m-1 px-4 py-2 bg-blue-500 text-white rounded"
                on:click=move |_| set_current_mode(NodeType::End)>"Select End"</button>
                <button
                class="m-1 px-4 py-2 bg-blue-500 text-white rounded"
                on:click=move |_| set_current_mode(NodeType::Wall)>"Draw Walls"</button>
                <button
                    class="m-1 px-4 py-2 bg-blue-500 text-white rounded"
                    on:click=generate_labyrinth
                    disabled={move || start_node.get().is_none() || end_node.get().is_none()}
                >
                    "Generate Labyrinth"
                </button>
                <button
                class="m-1 px-4 py-2 bg-blue-500 text-white rounded"
                on:click=clear_grid>"Clear Grid"</button>
                // <button
                // class="px-4 py-2 bg-blue-500 text-white rounded"
                // on:click=visualize_pathfinding disabled=is_animating>"Visualize Pathfinding"</button>
            </div>
            <p>
                "Start: " {move || start_node.get().map(|(r, c)| format!("({}, {})", r, c)).unwrap_or_else(|| "Not set".to_string())}
            </p>
            <p>
                "End: " {move || end_node.get().map(|(r, c)| format!("({}, {})", r, c)).unwrap_or_else(|| "Not set".to_string())}
            </p>
                <div class="speed-control" style="margin-top: 10px;">
                    <label>"Animation Speed: "</label>
                    <select on:change=move |ev| {
                        set_animation_speed(match event_target_value(&ev).as_str() {
                            "slow" => AnimationSpeed::Slow,
                            "fast" => AnimationSpeed::Fast,
                            _ => AnimationSpeed::Medium,
                        });
                    }>
                        <option value="slow">"Slow"</option>
                        <option value="medium" selected="true">"Medium"</option>
                        <option value="fast">"Fast"</option>
                    </select>
                </div>
        </div>
    }
}

fn clear_path_and_visited(grid: &mut Vec<Vec<Node>>) {
    for row in grid.iter_mut() {
        for node in row.iter_mut() {
            if node.node_type == NodeType::Path || node.node_type == NodeType::Visited {
                node.node_type = NodeType::Empty;
            }
        }
    }
}

async fn animate_path_finding(
    set_grid: WriteSignal<Vec<Vec<Node>>>,
    visited_nodes: ReadSignal<Vec<(usize, usize)>>,
    path: ReadSignal<Vec<(usize, usize)>>,
    set_is_animating: WriteSignal<bool>,
    animation_speed: ReadSignal<AnimationSpeed>,
) {
    let speed = match animation_speed.get() {
        AnimationSpeed::Slow => 50,
        AnimationSpeed::Medium => 20,
        AnimationSpeed::Fast => 5,
    };

    let batch_size = match animation_speed.get() {
        AnimationSpeed::Slow => 1,
        AnimationSpeed::Medium => 3,
        AnimationSpeed::Fast => 5,
    };

    let start = visited_nodes.get().first().cloned();
    let end = path.get().last().cloned();
    let visited = visited_nodes.get();

    for chunk in visited.chunks(batch_size) {
        set_grid.update(|g| {
            for &(row, col) in chunk {
                if Some((row, col)) != start && Some((row, col)) != end {
                    g[row][col].node_type = NodeType::Visited;
                }
            }
        });
        TimeoutFuture::new(speed).await;
    }

    for chunk in path.get().chunks(batch_size) {
        set_grid.update(|g| {
            for &(row, col) in chunk {
                if Some((row, col)) != start && Some((row, col)) != end {
                    g[row][col].node_type = NodeType::Path;
                }
            }
        });
        TimeoutFuture::new(speed).await;
    }

    set_grid.update(|g| {
        for &(row, col) in &visited_nodes.get() {
            if Some((row, col)) != start && Some((row, col)) != end && !path.get().contains(&(row, col)) {
                g[row][col].node_type = NodeType::Empty;
            }
        }
    });
    set_is_animating(false);
}

fn generate_maze(grid: &mut Vec<Vec<Node>>, start: (usize, usize), end: (usize, usize)) {
    let mut rng = rand::thread_rng();
    let mut stack = vec![start];
    let mut visited = HashSet::new();
    visited.insert(start);

    while let Some(current) = stack.pop() {
        let (row, col) = current;
        grid[row][col].node_type = NodeType::Empty;

        let mut directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        directions.shuffle(&mut rng);

        for (dr, dc) in directions {
            let next_row = row as i32 + dr * 2;
            let next_col = col as i32 + dc * 2;

            if next_row >= 0 && next_row < ROWS as i32 && next_col >= 0 && next_col < COLS as i32 {
                let next_row = next_row as usize;
                let next_col = next_col as usize;

                if !visited.contains(&(next_row, next_col)) {
                    visited.insert((next_row, next_col));
                    stack.push((next_row, next_col));
                    
                    let mid_row = (row as i32 + dr) as usize;
                    let mid_col = (col as i32 + dc) as usize;
                    grid[mid_row][mid_col].node_type = NodeType::Empty;
                }
            }
        }
    }

    for _ in 0..((ROWS * COLS) / 50) {  
        let row = rng.gen_range(1..ROWS-1);
        let col = rng.gen_range(1..COLS-1);
        grid[row][col].node_type = NodeType::Empty;
    }

    grid[start.0][start.1].node_type = NodeType::Empty;
    grid[end.0][end.1].node_type = NodeType::Empty;

    ensure_path(grid, start, end);
}

fn ensure_path(grid: &mut Vec<Vec<Node>>, start: (usize, usize), end: (usize, usize)) {
    let mut current = start;
    while current != end {
        let (row, col) = current;
        let (end_row, end_col) = end;

        if row < end_row {
            current = (row + 1, col);
        } else if row > end_row {
            current = (row - 1, col);
        } else if col < end_col {
            current = (row, col + 1);
        } else if col > end_col {
            current = (row, col - 1);
        }

        if current != end { 
            grid[current.0][current.1].node_type = NodeType::Empty;
        }
    }
}

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
    drawing_mode: DrawingMode,
) {
    let current_pos = (row, col);
    let grid_val = grid.get();
    let current_node_type = &grid_val[row][col].node_type;

    // Don't modify start or end nodes
    if start_node.get() == Some(current_pos) || end_node.get() == Some(current_pos) {
        return;
    }

    match current_mode.get() {
        NodeType::Wall => {
            if current_node_type != &NodeType::Start && current_node_type != &NodeType::End {
                match drawing_mode {
                    DrawingMode::Drawing => {
                        set_grid.update(|g| g[row][col].node_type = NodeType::Wall);
                    },
                    DrawingMode::Erasing => {
                        set_grid.update(|g| g[row][col].node_type = NodeType::Empty);
                    },
                    DrawingMode::None => {}
                }
            }
        },
        NodeType::Start => {
            if current_node_type != &NodeType::End {
                if let Some(old_start) = start_node.get() {
                    set_grid.update(|g| g[old_start.0][old_start.1].node_type = NodeType::Empty);
                }
                set_grid.update(|g| g[row][col].node_type = NodeType::Start);
                set_start_node(Some(current_pos));
            }
        },
        NodeType::End => {
            if current_node_type != &NodeType::Start {
                if let Some(old_end) = end_node.get() {
                    set_grid.update(|g| g[old_end.0][old_end.1].node_type = NodeType::Empty);
                }
                set_grid.update(|g| g[row][col].node_type = NodeType::End);
                set_end_node(Some(current_pos));
            }
        },
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

fn swarm(grid: &Vec<Vec<Node>>, start: (usize, usize), end: (usize, usize)) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut visited_nodes = Vec::new();
    let mut distances = vec![vec![usize::MAX; COLS]; ROWS];
    let mut prev = vec![vec![None; COLS]; ROWS];
    let mut priority_queue = BinaryHeap::new();
    
    distances[start.0][start.1] = 0;
    priority_queue.push(State { cost: 0, position: start });

    while let Some(State { position, .. }) = priority_queue.pop() {
        let (row, col) = position;
        
        if position == end {
            break;
        }

        // Add to visited nodes only if we haven't visited it before
        if !visited_nodes.contains(&position) {
            visited_nodes.push(position);
        }

        // Get neighbors with multiple passes
        for pass in 0..2 {  // Two passes for swarm-like behavior
            for (dr, dc) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let new_row = row as i32 + dr;
                let new_col = col as i32 + dc;

                if new_row >= 0 && new_row < ROWS as i32 && new_col >= 0 && new_col < COLS as i32 {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;

                    if grid[new_row][new_col].node_type != NodeType::Wall {
                        let distance_to_end = manhattan_distance((new_row, new_col), end);
                        let new_cost = distances[row][col] + 1 + (distance_to_end / 2);

                        if new_cost < distances[new_row][new_col] {
                            distances[new_row][new_col] = new_cost;
                            prev[new_row][new_col] = Some((row, col));
                            priority_queue.push(State {
                                cost: new_cost,
                                position: (new_row, new_col),
                            });
                        }
                    }
                }
            }
        }
    }

    let path = reconstruct_path(&prev, end);
    (visited_nodes, path)
}

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    ((a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()) as usize
}