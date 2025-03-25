pub type Position = (usize, usize);

pub fn get_neighbors(pos: Position, maze: &Vec<Vec<u8>>) -> Vec<Position> {
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut neighbors = Vec::new();

    for (dy, dx) in directions.iter() {
        let new_y = pos.0 as isize + dy;
        let new_x = pos.1 as isize + dx;
        if new_y >= 0
            && new_y < maze.len() as isize
            && new_x >= 0
            && new_x < maze[0].len() as isize
            && maze[new_y as usize][new_x as usize] == 0
        {
            neighbors.push((new_y as usize, new_x as usize));
        }
    }

    neighbors
}
