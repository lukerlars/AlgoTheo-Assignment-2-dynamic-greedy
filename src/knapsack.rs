use rand::{Rng};

pub fn run_discrete_knapsack(){
let valuables = generate_knapsack_01();
let potential_yields = solve_knapsack_01(&valuables, 14);
for i in potential_yields{    
    println!("{:?}",i);
}

}


#[derive(Clone, Debug)]
pub struct Knapsack{
    size_left : i32,
    items : Vec<i32>,
    total_value : i32
}

#[derive(Debug, Clone, Copy)]
pub struct ValuableItem {
    id: i32,
    size: i32,
    value : i32
}



pub fn generate_knapsack_01() -> Vec<ValuableItem> {
    let mut rng = rand::thread_rng();
    let n_items = rng.gen_range(0..10);

    let mut out :Vec<ValuableItem> = Vec::new();
    for id in 0..n_items{
       out.push(ValuableItem { id: id, size: (rng.gen_range(0..10)), value: (rng.gen_range(0..10)) })
    }
    out
}


pub fn solve_knapsack_01(items : & [ValuableItem] , knapsack_size : i32 )-> Vec<Knapsack>{
    
    let mut potential_knapsacks : Vec<Knapsack>= Vec::new();
    let knapsack_inital = Knapsack{size_left: knapsack_size, items : Vec::new(), total_value: 0};
    knapsack_recurse(items, &mut potential_knapsacks, &knapsack_inital,0,0);        

    let mut output : Vec<Knapsack>= Vec::new();
    potential_knapsacks.sort_by(|a,b| b.total_value.cmp(&a.total_value));

    let first_top = &potential_knapsacks[0];
    output.push(first_top.clone());

    for candidate in potential_knapsacks.iter().skip(1){
        if candidate.total_value == first_top.total_value{
            output.push(candidate.clone())
        } else {
            break;
        }
    }
   output 
}

fn knapsack_recurse(items: &[ValuableItem], knapsacks: &mut Vec<Knapsack>, current_knapsack: &Knapsack, start_index: usize, current_value: i32) {
//  recursive call performing bottom up logic for solving discrete knapsack problem. 
// Since adding new loot to the knapsack is commutative, we can perfrom a bottom up approach.  
   
    for (idx, &item) in items.iter().enumerate().skip(start_index) { // for an item j, if j has been checked by i, we do not need to check j by i.
        let new_value = current_value + item.value;
        if item.size <= current_knapsack.size_left {
            let mut new_knapsack = current_knapsack.clone();
            new_knapsack.size_left -= item.size;
            new_knapsack.items.push(item.id);
            new_knapsack.total_value += item.value;

            knapsacks.push(new_knapsack.clone());
            knapsack_recurse(items, knapsacks, &new_knapsack, idx + 1, new_value);
        }
    }
}



#[derive(Debug)]
pub struct ValuableLiquid{
    volume : f32,
    value : f32
}

pub fn generate_knapsack_frac() -> Vec<ValuableLiquid>{
    let mut rng = rand::thread_rng();
    let n_items = rng.gen_range(0..10);

    let mut out :Vec<ValuableLiquid> = Vec::new();
    for _ in 0..n_items{
       out.push(ValuableLiquid { volume: (rng.gen_range(0.0..10.0)), value: (rng.gen_range(0.0..10.0)) })
    }
    out
}

pub fn solve_knapsack_frac(items : &mut Vec<ValuableLiquid> , knapsack_size : f32 )-> (f32,f32){
    items.sort_by(|a,b| (b.value/b.volume).partial_cmp(&(a.value/a.volume)).unwrap());

    let mut cum_value = 0.0;
    let mut cum_amount  = 0.0; 
    
    for item in items.iter(){
        let space_left = knapsack_size - cum_amount;
        if space_left <=0.0{
            break;
        }
        else {
            let take = f32::min(item.volume, space_left);
            cum_amount += take;
            cum_value += (item.value/item.volume)*take;
        }
    }
    (cum_amount,cum_value)
}

pub fn run_frac_knapsack_demo(){
    let mut valuables = vec![ValuableLiquid{volume : 5.0, value : 5.0},ValuableLiquid{volume : 4.0, value : 7.0},ValuableLiquid{volume :8.0 , value : 2.0}];
    let knapsack_size = 6.0;
    let (cum_amount, cum_value) = solve_knapsack_frac(&mut valuables, knapsack_size);
    println!("{} {}", cum_amount, cum_value);
    let mut some_problem = generate_knapsack_frac();
    

    for valubable in some_problem.iter(){
        println!("{:?}",valubable);

    }
    let (some_amount, some_value) = solve_knapsack_frac(&mut some_problem, knapsack_size);
    println!("size of yield {}, total value {}",some_amount, some_value);

}


