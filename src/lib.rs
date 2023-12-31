use std::cmp::max;

use knapsack_utils::{
    Item,
    sum_weights,
    select_items,
    SearchResult,
};


pub fn solve_dp(items: &[Item], weight: usize) -> Result<SearchResult, ()> {
    let rows = items.len() + 1;
    let columns = (weight + 1) as usize;
    let mut array = vec![vec![0; columns]; rows];

    // Solve knapsack problem
    // Answer is in the lower right corner of the array.
    for (prev_row, row) in (1..rows).enumerate() {
        let it = &items[prev_row];
        for col in 1..it.weight {
            array[row][col] = array[prev_row][col];
        }
        for col in it.weight..columns {
            array[row][col] = max(
                array[prev_row][col],
                it.value + array[prev_row][col - it.weight],
            )
        }
    }

    // Work backwards to find the path
    // Vector of indexes to items are stored in reverse order in revese_path
    let mut reverse_path: Vec<usize> = vec![];
    let mut col: usize = columns - 1;
    for row in (1..rows).rev() {
        if array[row][col] > array[row - 1][col] {
            // Item in row `row - 1` was added at this step.
            reverse_path.push(row - 1);
            col -= items[row - 1].weight;
        }
    }

    // Reverse the path into a forward path
    let forward_path: Vec<usize> = reverse_path.into_iter()
        .rev()
        .collect::<Vec::<_>>();

    let selected_items: Vec<Item> = select_items(items, &forward_path); 
    let weight = sum_weights(&selected_items); 

    Ok((forward_path, weight, array[rows - 1][columns - 1]))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dp_1_1() {
        // Sorted by weight
        let items = vec![
            Item{value:1500, weight:1},
            Item{value:2000, weight:3},
            Item{value:3000, weight:4},
        ];
        if let Ok(result) = solve_dp(&items, 4) {
            assert_eq!(result.2, 3500);
            assert_eq!(result.1, 4);
        }
    }

    #[test]
    fn dp_1_2() {
        // Reverse sorted by weight
        let items = vec![
            Item{value:3000, weight:4},
            Item{value:2000, weight:3},
            Item{value:1500, weight:1},
        ];
        if let Ok(result) = solve_dp(&items, 4) {
            assert_eq!(result.2, 3500);
            assert_eq!(result.1, 4);
        }
    }

    #[test]
    fn dp_1_3() {
        // Sorted by weight
        let items = vec![
            Item{value:1500, weight:1},
            Item{value:2000, weight:3},
            Item{value:3000, weight:4},
        ];
        if let Ok(result) = solve_dp(&items, 4) {
            assert_eq!(result.2, 3500);
            assert_eq!(result.1, 4);
        }
    }

    #[test]
    fn dp_2_1() {
        // Sorted by weight
        let items = vec![
            Item{value:2, weight:1},
            Item{value:3, weight:2},
            Item{value:1, weight:2},
        ];
        if let Ok(result) = solve_dp(&items, 3) {
            assert_eq!(result.2, 5);
            assert_eq!(result.1, 3);
        }
    }

    #[test]
    fn dp_2_2() {
        // Reverse sorted by weight
        let items = vec![
            Item{value:1, weight:2},
            Item{value:3, weight:2},
            Item{value:2, weight:1},
        ];
        if let Ok(result) = solve_dp(&items, 3) {
            assert_eq!(result.2, 5);
            assert_eq!(result.1, 3);
        }
    }

    #[test]
    fn dp_2_3() {
        // Sorted by weight
        let items = vec![
            Item{value:2, weight:1},
            Item{value:1, weight:2},
            Item{value:3, weight:2},
        ];
        if let Ok(result) = solve_dp(&items, 3) {
            assert_eq!(result.2, 5);
            assert_eq!(result.1, 3);
        }
    }

    #[test]
    fn dp_2_4() {
        // Reverse sorted by weight
        let items = vec![
            Item{value:3, weight:2},
            Item{value:1, weight:2},
            Item{value:2, weight:1},
        ];
        if let Ok(result) = solve_dp(&items, 3) {
            assert_eq!(result.2, 5);
            assert_eq!(result.1, 3);
        }
    }

    #[test]
    fn dp_2_5() {
        // Sorted by value
        let items = vec![
            Item{value:1, weight:2},
            Item{value:2, weight:1},
            Item{value:3, weight:2},
        ];
        if let Ok(result) = solve_dp(&items, 3) {
            assert_eq!(result.2, 5);
            assert_eq!(result.1, 3);
        }
    }

    #[test]
    fn dp_2_6() {
        // Reverse-sorted by value
        let items = vec![
            Item{value:3, weight:2},
            Item{value:2, weight:1},
            Item{value:1, weight:2},
        ];
        if let Ok(result) = solve_dp(&items, 3) {
            assert_eq!(result.2, 5);
            assert_eq!(result.1, 3);
        }
    }
}
