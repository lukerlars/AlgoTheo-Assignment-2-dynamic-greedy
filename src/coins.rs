
use std::collections::HashMap;


pub fn greedy_coin_minimizer(coins: &mut [i32], target_sum : i32 )-> Vec<(i32, i32)>{
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

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct CoinStack{
    coin: i32,
    count: i32
}

#[derive(Debug, Clone)]
pub struct CoinPouch{
    // representing a set of coin stacks
    value: i32,
    count: i32,
    pub coin_stacks: Vec<CoinStack>,
}



pub fn topdown_coin_minimizer(coins: &mut [i32], target_sum : i32)->Option<CoinPouch>{
    coins.sort(); 
    coins.reverse(); //descending

    assert_eq!(coins[coins.len()-1], 1); // chek that coin array contains a 1

    let mut pouches : Vec<CoinPouch> = coins[..coins.len()-1].iter() // no need to include last, since it is a 1
                    .map(|&x| CoinPouch{
                                            value :x/target_sum,  
                                            count : x/target_sum,
                                            coin_stacks :Vec::new(),
                                            }).collect();
    
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




#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct CoinSum{
    val: i32,
    pub coins : Vec<CoinStack>
}

pub fn coin_recurse(coins: &[i32], node_val:i32 ,  optimal_sums: &mut  HashMap<i32,CoinSum>)-> CoinSum{
    // Dynamic and recursice calculation of the minimum coin sum of a given value. 
    
    // println!("coins: {:?} node_value {:?}, optimal_sums {:?}",coins, node_val, optimal_sums );
    if let Some(optimal_sum) = optimal_sums.get(&node_val){
        return optimal_sum.clone();
    }
    if coins.len() == 1{
        assert_eq!(coins[0], 1); // check if equall to one. No option (yet) for coins without a 1 coin
        let leaf_sum= CoinSum{ val: node_val, coins: vec![CoinStack{coin :1, count: node_val}]};
        optimal_sums.insert(node_val, leaf_sum.clone());
        return leaf_sum; 
        }

    let mut min_coin = 0; 
    let mut min_coin_score = i32::MAX;
    let mut min_child_sum : Option<CoinSum> = None;
    for (idx,&coin ) in coins.iter().enumerate() {
        if node_val == coin{
            let leaf_sum= CoinSum{ val: node_val, coins: vec![CoinStack{coin :coin, count: 1}]};
            optimal_sums.insert(node_val, leaf_sum.clone());
            return leaf_sum; // When value equals a coin value means we've reached a leaf node
        }
        else if node_val < coin {
          return coin_recurse(&coins[idx+1..], node_val, optimal_sums) 
          } 
    
        else {
        let child_val = node_val - coin;
        let child_sum= coin_recurse(&coins[idx..],child_val,  optimal_sums);
        let child_score = child_sum.coins.iter().map(|x| x.count).sum();
        if child_score < min_coin_score{ // this will be the case at least once because of sentinel number
            min_coin_score = child_score;
            min_child_sum = Some(child_sum);
            min_coin = coin;
            }
        }
    }
    let mut out_sum = min_child_sum.unwrap();
    out_sum.val += min_coin;
    let mut insert_flag = false;
    for stack in out_sum.coins.iter_mut(){
        if min_coin == stack.coin{
            stack.count+=1;
            insert_flag = true;
            break;
        }}
    if !insert_flag {
        out_sum.coins.push(CoinStack{coin: min_coin, count : 1});
    }
    optimal_sums.insert(node_val,out_sum.clone());
    out_sum
} 


pub fn dynamic_coin_solver(coins: &mut [i32],sum_value: i32  )-> CoinSum{
   //Solve the minimal coin sum problem by deploying recursive function 
    coins.sort();
    coins.reverse();
    let mut optimal_sums:  HashMap<i32,CoinSum> = HashMap::new();
    coin_recurse(&coins, sum_value, &mut optimal_sums);
    optimal_sums.get(&sum_value).unwrap().clone()
}


 