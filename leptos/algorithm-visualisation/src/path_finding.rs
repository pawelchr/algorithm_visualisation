use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq)]
pub struct TraceStep {
    pub current: u32,
    pub frontier: Vec<u32>,
    pub visited: Vec<u32>,
}

pub fn trace(from: u32, to: u32, v: &Vec<Vec<u32>>) -> Vec<TraceStep> {
    let mut frontier: VecDeque<u32> = VecDeque::new();
    let mut visited: Vec<u32> = vec![0xffff; v.len()];
    let mut steps: Vec<TraceStep> = Vec::new();
    
    frontier.push_back(from);
    visited[from as usize] = from;
    
    while let Some(p) = frontier.pop_front() {
        steps.push(TraceStep {
            current: p,
            frontier: frontier.clone().into(),
            visited: visited.clone(),
        });
        
        if p == to {
            break;
        }
        
        for &n in &v[p as usize] {
            if visited[n as usize] == 0xffff {
                visited[n as usize] = p;
                frontier.push_back(n);
            }
        }
    }
    
    // Reconstruct path
    let mut path = Vec::new();
    let mut p = to;
    while p != from {
        path.push(p);
        p = visited[p as usize];
    }
    path.push(from);
    path.reverse();
    
    // Add final step with the complete path
    steps.push(TraceStep {
        current: to,
        frontier: path.clone(),
        visited: visited,
    });
    
    steps
}

pub fn gen_field_graph(width: u32, height: u32) -> Vec<Vec<u32>> {
    let mut v: Vec<Vec<u32>> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let mut row: Vec<u32> = Vec::new();
            let pos = x + y * width;

            // Check west neighbor
            if x > 0 {
                row.push(pos - 1);
            }

            // Check south neighbor
            if y < height - 1 {
                row.push(pos + width);
            }

            // Check east neighbor
            if x < width - 1 {
                row.push(pos + 1);
            }

            // Check north neighbor
            if y > 0 {
                row.push(pos - width);
            }

            v.push(row);
        }
    }
    v
}
