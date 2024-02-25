// Main file, for reporting and demoing each solution to problems  


mod matrix_chain;
use matrix_chain::{matrix_chain_order, print_optimal_parens};

mod knapsack;
use knapsack::{ValuableItem, ValuableLiquid, generate_knapsack_frac, generate_knapsack_01, solve_knapsack_01, solve_knapsack_frac};

mod coins;
use coins::{greedy_coin_minimizer, topdown_coin_minimizer, dynamic_coin_solver};

use std::{time::Instant};
use rand::Rng;

fn main(){
    // run all demonstration functions. Execution time reported for each solutoin
    run_matchain_demo();
    run_discrete_knapsack();
    run_frac_knapsack_demo();
    run_greedy_coin_demo();
    demo_coin_minimizer_toppdown();
    demo_dynamic_coin_solver();
    compare_dynami_greedy_coin_solver();
}


// Demo for matrix chain. Task 1.2 and 1.3

fn run_matchain_demo() {
    //Demo for the matrix chain aglorithm
    
    let dimensions: Vec<i32> = vec![3,4,5,4,3,4];
    let start = Instant::now();
    let (res_m, res_s) = matrix_chain_order(&dimensions);
    let duration= start.elapsed();
    print!("Min number of computations with optimal parenthesization: {}",res_m[0][4]);
    print!("\n And that paranthesization is: ");
    print_optimal_parens(&res_s, 0, 4);
    println!("Running time: {:?}", duration);
    }


// Task 2.1 run discrete knapsack

fn run_discrete_knapsack(){
    let valuables = generate_knapsack_01();
    println!("\n Generated discrete knapsack problem, Valuable items :");
    for valuable in valuables.iter(){
        println!("{:?}",valuable)};
    let start = Instant::now();
    let potential_yields = solve_knapsack_01(&valuables, 14);
    let duration= start.elapsed();
    
    println!("Found solution to knapsack problem :");
    for i in potential_yields{    
       println!("{:?}",i);
     }
    println!("Runtime {:?}", duration);
}

// Task 2.2 Fractional knapsack demo

fn run_frac_knapsack_demo(){
    // Demo of the fractional knapsack problem
    // let mut valuables = vec![ValuableLiquid{volume : 5.0, value : 5.0},ValuableLiquid{volume : 4.0, value : 7.0},ValuableLiquid{volume :8.0 , value : 2.0}];
    let knapsack_size = 6.0;
    // let (cum_amount, cum_value) = solve_knapsack_frac(&mut valuables, knapsack_size);
    // println!("{} {}", cum_amount, cum_value);
    
    let mut some_problem = generate_knapsack_frac();
    println!("\n Generated liquid knapsack problem:");
    for valubable in some_problem.iter(){
        println!("{:?}",valubable);

    }

    let start = Instant::now();
    let (some_amount, some_value) = solve_knapsack_frac(&mut some_problem, knapsack_size);
    let duration= start.elapsed();
    println!("Found solution");
    println!("Knapsack size {}, size of yield {}, total value {}",knapsack_size ,some_amount, some_value);
    println!("Duation {:?}", duration);
}

// Task 3.1 greedy coin sum solver

pub fn run_greedy_coin_demo(){
    println!{"\n Greedy coin sum solver demo"};
    let mut coins = vec![1,3,5,10];
    let start = Instant::now();
    let result = greedy_coin_minimizer(&mut coins, 99);
    let duation = start.elapsed();
    println!("found solution for target sum {}, using coins {:?}, with a greedy approach:", 99, coins);
    for res in result{
        println!("coin value: {}, counts: {}", res.0, res.1);
        }
    println!("Duration {:?}", duation);
    }
 
// Task 3.1 (2) Greedy coin solver, topdown approach

 pub fn demo_coin_minimizer_toppdown(){
        println!("\n Demo coin sum minimzer top down appoach");
        let mut coins = vec![1,5,10,20];
        let start = Instant::now();
        let result = topdown_coin_minimizer(&mut coins, 15);
        let duration = start.elapsed();
    
        println!("Found solution to sum target {}, with coins {:?}", 15, coins);
        match result{
            Some(val) => println!("Found the best coin pouch to be {:?}", val),
            None => println!("something went wrong here")
        } 
        }

// Task 3.3, 3.4 Dynamic coin solver

fn demo_dynamic_coin_solver(){
    println!("\n Demo dynamic coin solver");
    let mut coins = vec![1,5,11];
    let sum_value = 15;
    println!("find optimal sum of value {}, using coins: {:?}", 15, coins);
    let start = Instant::now();
    let res = dynamic_coin_solver(&mut coins, sum_value);
    let duration = start.elapsed();
    println!("Found solution {:?}", res);
    println!("Duration {:?}", duration);


    let mut nok = vec![1,5,10,20];
    let start = Instant::now();
    let nok_res = dynamic_coin_solver(&mut nok,127); 
    let duration = start.elapsed();
    println!("Test, norwegian coins: {:?}", nok);
    println!("Value {}, found sum {:?}",127, nok_res ); 
    println!("Time elapsed {:?}",duration);
}

fn compare_dynami_greedy_coin_solver(){
   
    for _ in 0..10 {    
        let mut coins = vec![1,5,10,20];
        let mut rng = rand::thread_rng();
        let target_sum = rng.gen_range(1..300);
        println!("\n Compare greedy vs dynamic coin solver for norwegian coins: {:?}, and target sum {}", coins, target_sum);
    
        let start = Instant::now();
        let result = topdown_coin_minimizer(&mut coins, target_sum);
        let duration = start.elapsed();

        println!("Greedy approach results:"); // {:?}", result);
        for res in result.unwrap().coin_stacks.iter(){
            println!("{:?}", res);
        }
        println!("Elapsed time {:?}", duration);

        let start = Instant::now();
        let nok_res = dynamic_coin_solver(&mut coins,target_sum);
        let duration = start.elapsed();
        println!("\nDynamic approach: " ); 
        for res in nok_res.coins.iter().rev(){
            println!("{:?}", res);
        }
        println!("Time elapsed {:?}",duration);
        }
}