# Game of Life Rules:
1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
2. Any live cell with two or three live neighbours lives on to the next generation.
3. Any live cell with more than three live neighbours dies, as if by overpopulation.
4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

## Program Outline
- Need to track two game states: current and next.
- States are represented as hashmaps: 
    - point (x, y) -> # live cell neighbors
    - all cells with at least one neighbor will be tracked
    - when a cell dies it is removed from the hashmap
- update state fxn will be needed.
    - count the number of neighbors that each cell has.
    - follow the rules for removing, retaining or adding cells.
    - when a cell is added or removed all surrounding cells will need to be
        updated as their number of neighbors has changed.
