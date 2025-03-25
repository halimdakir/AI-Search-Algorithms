use std::collections::{HashMap, HashSet, VecDeque};
use crate::utils::{get_neighbors, Position};

// Breadth-First Search
pub fn bfs(maze: &Vec<Vec<u8>>, start: Position, goal: Position) -> Option<Vec<Position>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut path: HashMap<Position, Position> = HashMap::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
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

        for neighbor in get_neighbors(current, maze) {
            if !visited.contains(&neighbor) {
                queue.push_back(neighbor);
                visited.insert(neighbor);
                path.insert(neighbor, current);
            }
        }
    }

    None
}
