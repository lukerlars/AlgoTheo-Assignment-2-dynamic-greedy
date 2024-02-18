// mod matrix_chain;
// use matrix_chain::{matrix_chain_order,run_matchain_demo};

// mod knapsack;
// use knapsack::{run_discrete_knapsack, run_frac_knapsack_demo};


fn main(){
let mut coins = vec![1,3,5,10];
let result = greedy_coin_minimizer(&mut coins, 99);
for res in result{
    println!("coin value: {}, counts: {}", res.0, res.1);
    }
}



fn greedy_coin_minimizer(coins: &mut [i32], target_sum : i32 )-> Vec<(i32, i32)>{
    // greedy solution to coins summing up to a given target value
   coins.sort();
   coins.reverse();
   let mut target_sum_diff = target_sum;
   let mut coin_counts : Vec<(i32, i32)> = Vec::new();
   for coin in coins{
        let mut coin_count = 0;
        while *coin <= target_sum_diff {
            target_sum_diff -= *coin;
            coin_count += 1; 
        }
        coin_counts.push((coin.clone(), coin_count))
   }
   coin_counts
}