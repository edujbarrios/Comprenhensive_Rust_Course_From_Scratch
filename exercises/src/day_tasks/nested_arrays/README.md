
## Solution Approach: Transposing a 3x3 Matrix in Rust

To **transpose a matrix**, we flip its rows into columns. That means the element at position `(i, j)` in the original matrix becomes position `(j, i)` in the transposed one.

For example:
```text
Original:
1 2 3
4 5 6
7 8 9

Transposed:
1 4 7
2 5 8
3 6 9
```

### Steps:

1. **Define a new matrix** to store the transposed result. It should be the same size as the input (3x3).
2. **Loop through each row and column** using nested `for` loops.
3. **Swap rows with columns** by assigning `result[j][i] = matrix[i][j]`.

This method only works with **3x3 matrices** as required by the exercise.