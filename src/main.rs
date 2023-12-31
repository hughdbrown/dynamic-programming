use prng::Prng;
use knapsack_utils::{
    make_items,
    sum_values,
    sum_weights,
    select_items,
    Item,
};
use dynamic_programming::solve_dp;

const NUM_ITEMS: usize = 200;

const MIN_VALUE: u64 = 1;
const MAX_VALUE: u64 = 12;
const MIN_WEIGHT: usize = 4;
const MAX_WEIGHT: usize = 15;
const LIMIT_WEIGHT: usize = 1500; 

fn main() {
    let mut prng = Prng::new();
    let items = make_items(
        &mut prng,
        NUM_ITEMS,
        MIN_VALUE, MAX_VALUE,
        MIN_WEIGHT, MAX_WEIGHT,
    );

    println!("*** Parameters ***");
    println!("# items:        {}", NUM_ITEMS);
    println!("Total value:    {}", sum_values(&items));
    println!("Total weight:   {}", sum_weights(&items));
    println!("Allowed weight: {}", LIMIT_WEIGHT);
    println!();

    let result = solve_dp(&items, LIMIT_WEIGHT);
    match result {
        Ok(solution) => {
            let (path, weight, value) = solution;
            println!("Weight = {weight}");
            println!("Value = {value}");
            println!("Path = {:?}", path);
            let selected_items: Vec<Item> = select_items(&items, &path);
            println!("Items = {:#?}", selected_items);
            println!("Validate calculations:\n\tweight = {}\n\t value = {}", sum_weights(&selected_items), sum_values(&selected_items));
        },
        Err(_) => {},
    }
}
