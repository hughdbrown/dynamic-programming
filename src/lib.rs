use std::cmp::max;

use knapsack_utils::{
    Item,
};


fn solve_dp(items: &[Item], weight: u64) -> u64 {
    let rows = items.len() + 1;
    let columns = (weight + 1) as usize;
    let mut array = vec![vec![0; columns]; rows];
    for i in 1..rows {
        let it = &items[i - 1];
        for j in 1..it.weight {
            array[i][j] = array[i - 1][j];
        }
        for j in it.weight..columns {
            array[i][j] = max(
                array[i - 1][j],
                it.value + array[i - 1][j - it.weight],
            )
        }
    }

    println!("{:?}", array);
    let mut path: Vec<Item> = vec![];
    let mut col: usize = columns - 1;
    for row in (1..rows).rev() {
        if array[row][col] > array[row - 1][col] {
            // Item was added at this step.
            let it = &items[row - 1];
            path.push(it.clone());
            col -= it.weight;
        }
    }
    println!("path = {:?}", &path.iter().rev().collect::<Vec::<_>>());

    array[rows - 1][columns - 1]
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
        let value = solve_dp(&items, 4);
        assert_eq!(value, 3500);
    }

    #[test]
    fn dp_1_2() {
        // Reverse sorted by weight
        let items = vec![
            Item{value:3000, weight:4},
            Item{value:2000, weight:3},
            Item{value:1500, weight:1},
        ];
        let value = solve_dp(&items, 4);
        assert_eq!(value, 3500);
    }

    #[test]
    fn dp_1_3() {
        // Sorted by weight
        let items = vec![
            Item{value:1500, weight:1},
            Item{value:2000, weight:3},
            Item{value:3000, weight:4},
        ];
        let value = solve_dp(&items, 4);
        assert_eq!(value, 3500);
    }

    #[test]
    fn dp_2_1() {
        // Sorted by weight
        let items = vec![
            Item{value:2, weight:1},
            Item{value:3, weight:2},
            Item{value:1, weight:2},
        ];
        let value = solve_dp(&items, 3);
        assert_eq!(value, 5);
    }

    #[test]
    fn dp_2_2() {
        // Reverse sorted by weight
        let items = vec![
            Item{value:1, weight:2},
            Item{value:3, weight:2},
            Item{value:2, weight:1},
        ];
        let value = solve_dp(&items, 3);
        assert_eq!(value, 5);
    }

    #[test]
    fn dp_2_3() {
        // Sorted by weight
        let items = vec![
            Item{value:2, weight:1},
            Item{value:1, weight:2},
            Item{value:3, weight:2},
        ];
        let value = solve_dp(&items, 3);
        assert_eq!(value, 5);
    }

    #[test]
    fn dp_2_4() {
        // Reverse sorted by weight
        let items = vec![
            Item{value:3, weight:2},
            Item{value:1, weight:2},
            Item{value:2, weight:1},
        ];
        let value = solve_dp(&items, 3);
        assert_eq!(value, 5);
    }

    #[test]
    fn dp_2_5() {
        // Sorted by value
        let items = vec![
            Item{value:1, weight:2},
            Item{value:2, weight:1},
            Item{value:3, weight:2},
        ];
        let value = solve_dp(&items, 3);
        assert_eq!(value, 5);
    }

    #[test]
    fn dp_2_6() {
        // Reverse-sorted by value
        let items = vec![
            Item{value:3, weight:2},
            Item{value:2, weight:1},
            Item{value:1, weight:2},
        ];
        let value = solve_dp(&items, 3);
        assert_eq!(value, 5);
    }
}
