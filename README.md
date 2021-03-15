# Game of Life Rules:
1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
2. Any live cell with two or three live neighbours lives on to the next generation.
3. Any live cell with more than three live neighbours dies, as if by overpopulation.
4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

# Program Outline
- Need to track two game states: current and next.
- States are represented as hashmaps: 
    - point (x, y) -> # live cell neighbors
    - only live cells will be tracked
    - when a cell dies it is removed from the hashmap
