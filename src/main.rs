// mod matrix_chain;
// use matrix_chain::{matrix_chain_order,run_matchain_demo};

// mod knapsack;
// use knapsack::{run_discrete_knapsack, run_frac_knapsack_demo};


fn run_greedy_coin_demo(){
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

#[derive(Debug, Clone)]
struct CoinStack{
    // representing a stack of one type of coin
    coin: i32,
    count: i32, 
}
#[derive(Debug, Clone)]
struct CoinPouch{
    // representing a set of coin stacks
    value: i32,
    coin_stacks: Vec<CoinStack>,
    count: i32,
}

fn bottomup_coin_minimizer(coins: &mut [i32], target_sum : i32)->Option<CoinPouch>{
    coins.sort(); 
    coins.reverse(); //descending

    assert_eq!(coins[coins.len()-1], 1); // chek that coin array contains a 1

    let mut pouches : Vec<CoinPouch> = coins[..coins.len()-1].iter() // no need to include last, since it is a 1
                    .map(|&x| CoinPouch{value :x/target_sum,  
                                            coin_stacks :Vec::new(),
                                            count : x/target_sum}).collect();
    
    for (id,pouch) in pouches.iter_mut().enumerate(){
        let mut diff = target_sum - pouch.value; 
        if diff <= 0 {break;}
        for next_coin in coins.iter().skip(id){
            let mut value = 0;
            let mut count = 0;
            while *next_coin <= diff {
               value += *next_coin;
               diff -= *next_coin;
               count += 1;
            }
            if count != 0 {
                pouch.value += value;
                pouch.count += count;
                pouch.coin_stacks.push(CoinStack{coin: next_coin.clone(),count : count})
            }
        }
    }
    pouches.iter().min_by_key(|pouch| pouch.count).cloned()
}

fn main(){
    let mut coins = vec![1,5,11];
    let result = bottomup_coin_minimizer(&mut coins, 15);

    match result{
        Some(val) => println!("Found the best coin pouch to be {:?}", val),
        None => println!("something went wrong here")
    } 
        
    }