
pub fn matrix_chain_order(matrix_dimensions: &[i32]) -> (Vec<Vec<i32>>, Vec<Vec<usize>>) {
    let n = matrix_dimensions.len() - 1;  
    let mut m: Vec<Vec<i32>> = vec![vec![i32::MAX; n]; n]; 
    let mut s: Vec<Vec<usize>> = vec![vec![0; n]; n]; 
 
    for i in 0..n {
        m[i][i] = 0;
    }
 
    for l in 2..=n {
        for i in 0..n - l + 1 {
            let j = i + l - 1;
            m[i][j] = i32::MAX;
            for k in i..j {
                let q = m[i][k] + m[k + 1][j] + matrix_dimensions[i] * matrix_dimensions[k + 1] * matrix_dimensions[j + 1];
                if q < m[i][j] {
                    m[i][j] = q;
                    s[i][j] = k;  
                }
            }
        }
    }
    (m, s)
 }


 pub fn print_optimal_parens(s: &Vec<Vec<usize>>,i : usize,j : usize){
    if i == j{ 
        print!("A{}", i)
    }
    else{
        print!("(");
        print_optimal_parens(s, i, s[i][j]);
        print_optimal_parens(s,s[i][j] +1 ,j);
        print!(")");
    }
 }


 
