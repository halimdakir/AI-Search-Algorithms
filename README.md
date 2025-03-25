# Maze Solver with Search Algorithms (Rust)

## Project Overview

This project demonstrates solving a maze using three AI classical pathfinding research algorithms:

- Breadth-First Search (BFS)
- Depth-First Search (DFS)
- A* Search Algorithm

The maze is implemented using a 2D vector in Rust. Each algorithm explores the grid to find a path from a start to an end point. The output is currently printed in the terminal, with optional future enhancements for ASCII or graphical visualization.

---

## Algorithms Used

### Breadth-First Search (BFS)
- Explores all neighbors level by level.
- Guarantees the shortest path in terms of number of steps.
- Uses a queue (FIFO) for traversal.

### Depth-First Search (DFS)
- Explores as deep as possible along each path before backtracking.
- Does not guarantee the shortest path.
- Uses a stack (LIFO) for traversal.

### A* Search
- Combines actual cost from the start with a heuristic to the goal.
- Uses Manhattan Distance as the heuristic function.
- Uses a priority queue to explore the most promising paths first.

---

## Maze Representation

The maze is represented as a 2D vector of integers:

- `0` represents an open path
- `1` represents a wall

### Start and End Points
- Start: `(0, 1)`
- End: `(9, 18)`

---

## Output and Visualization

The project outputs the maze path in the terminal as coordinate sequences for now. Visual output (ASCII or graphical) may be added later using crates like `crossterm` or SDL bindings.

---

## Technologies Used

- Rust 1.85+
- `std::collections` for BFS, DFS, A* structures
- Optional: Visualization using external crates (not yet implemented)

---

## How to Run

1. Clone the repository:
   ```bash
   git clone https://github.com/halimdakir/AI-Search-Algorithms.git
   cd AI-Search-Algorithms
