# Purpose
Produce dynamic programming solution to 0-1 knapsack problem.

# Related projects and repos
- https://github.com/hughdbrown/knapsack_utils
- https://github.com/hughdbrown/exhaustive-search
- https://github.com/hughdbrown/branch-and-bound
- https://github.com/hughdbrown/rods-technique
- https://github.com/hughdbrown/test-knapsack-algos

# Result
Very fast. Solves problem of size 200 in 0.50 seconds.

```
> time cargo run
Items:          200
Total value:    1171
Total weight:   1871
Allowed weight: 1500

Weight = 1500
Value = 1115
Path = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 18, 19, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 44, 45, 47, 48, 49, 50, 51, 53, 54, 55, 56, 58, 59, 61, 62, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 76, 77, 79, 80, 83, 84, 86, 87, 88, 89, 90, 91, 92, 93, 95, 96, 98, 99, 100, 102, 103, 104, 105, 106, 107, 109, 111, 112, 113, 114, 115, 116, 117, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 131, 132, 133, 136, 137, 138, 139, 140, 141, 143, 145, 146, 147, 149, 151, 152, 153, 154, 155, 156, 157, 158, 159, 164, 165, 168, 169, 170, 172, 173, 174, 175, 176, 177, 178, 179, 181, 182, 183, 184, 185, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199]
...
Validate calculations:
	weight = 1500
	 value = 1115
cargo run  0.09s user 0.03s system 25% cpu 0.457 total
```
