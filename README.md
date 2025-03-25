# Maze Solver with Search Algorithms (Rust)

## Project Overview

This project demonstrates solving a maze using three classical AI pathfinding algorithms implemented in Rust:

- Breadth-First Search (BFS)
- Depth-First Search (DFS)
- A* Search Algorithm

The maze is represented using a 2D vector. Each algorithm finds a path from a start point to an end point. Paths are displayed in the terminal and visualized using Python (for now), with potential for Rust-based visualization in future updates.

---

## Maze Representation

The maze is defined as a 2D vector of integers:

- `0` = walkable path
- `1` = wall (obstacle)

### Start and End Points:
- Start: `(0, 1)`
- End: `(9, 18)`

Maze visualization (walls in black, start in red, end in blue):

![Maze](https://github.com/user-attachments/assets/49c6b928-4e5d-4bb9-9e62-484edfd14747)

---

## Algorithms Used

### Breadth-First Search (BFS)
- Explores all neighbors level by level (layer-wise).
- Guarantees the shortest path.
- Uses a queue (FIFO).

**BFS path visualization:**

![BFS](https://github.com/user-attachments/assets/7c2f4139-dbe4-415f-9b32-daac856ab753)

---

### Depth-First Search (DFS)
- Explores as far as possible along each path before backtracking.
- Does not guarantee the shortest path.
- Uses a stack (LIFO).

**DFS path visualization:**

![DFS](https://github.com/user-attachments/assets/c1bc084e-a93f-4df9-8fdd-e1ba772989c8)

---

### A* Search Algorithm
- Uses both actual cost and heuristic cost (Manhattan Distance).
- Prioritizes paths most likely to reach the goal quickly.
- Uses a priority queue.

**A* path visualization:**

![A*](https://github.com/user-attachments/assets/1b4d51b6-8528-4b2a-a859-ad80f1624614)

---

## Output and Visualization

- Each algorithm prints the path coordinates to the terminal.
- Visualization is currently done using Python and `matplotlib` for clarity.
- Rust-native visual output (ASCII or graphical) may be added in the future using libraries like `plotters` or `crossterm`.

---

## Technologies Used

- Rust 1.85+
- Standard library: `VecDeque`, `HashSet`, `HashMap`, `BinaryHeap`
- Algorithms implemented from scratch
- Python used temporarily for visual verification of pathfinding

---

## How to Run

1. Clone the repository:
   ```bash
   git clone https://github.com/halimdakir/AI-Search-Algorithms.git
   cd AI-Search-Algorithms
