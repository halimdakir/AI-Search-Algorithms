use std::collections::{HashMap, HashSet};
use crate::utils::{get_neighbors, Position};

//Depth-First Search
pub fn dfs(maze: &Vec<Vec<u8>>, start: Position, goal:Position) -> Option<Vec<Position>>{
    let mut stack = Vec::new();
    let mut visited = HashSet::new();
    let mut path: HashMap<Position, Position> = HashMap::new();

    stack.push(start);

    while let Some(current) = stack.pop() {
        visited.insert(current);
        if current == goal {
            // Reconstruct path
            let mut full_path = vec![current];
            let mut prev = current;
            while let Some(&p) = path.get(&prev) {
                full_path.push(p);
                prev = p;
            }
            full_path.reverse();
            return Some(full_path);
        }

        for neighbor in get_neighbors(current, maze).into_iter().rev() {
            if !visited.contains(&neighbor) {
                stack.push(neighbor);
                path.insert(neighbor, current);
            }
        }
    }

    None

}