Link to github repo: https://github.com/lukerlars/AlgoTheo-Assignment-2-dynamic-greedy
source ->
		matrix_chain.rs (Code problem 1)
		knapsack.rs (Problem 2)
		coins.rs (problem 3)
		main.rs (Run demonstration of all problem solutions)
		
## Problem 1) 

Define a problem for matrix chain multiplication with 5 matrices. 
1) Solve the parenthesization problem by hand using the recursive formula to fill the memoization tables m and s.
2) Implement a dynamic programming program that solves the problem for any number of matrices. 
3) Does it exist a greedy choice that could apply to this problem?


### Answer
1) Proposing the matrices: $\{A_0, A_1,A_2, A_3, A_4\}$
with the dimensions:
$$
\begin{matrix} A_0 \rightarrow (3 \times 4)
\\ A_1 \rightarrow (4 \times 5)
\\ A_2 \rightarrow (5 \times 4)
   \\ A_3 \rightarrow (4 \times 3)
   \\ A_4 \rightarrow (3 \times 4)
   \end{matrix}
$$

We need to find the optimal parenthesization of $A_0 A_1A_2A_3,A_4$


Recall the recursive formula for the minimum number of scalar multiplications necessary for computing matrix $A_i$:

	$$ \tag{1} m[i,j] = \begin{cases}0 & \text{ if } i = j\\ min_{i \leqq k \leqq j }\{ m[i,k] + m[k+1,j] + p_{i-1}p_kp_j \}  & \text{ if } i < j \ \end{cases} 
	$$
	

Where $p$ denotes the column dimension of the matrix. 

We want to fill out the memoization tables $s$ and $m$, storing intermediate values of the 
recursion. Using equation $(1)$ we can fill out the tables. For the $m$ table, we can begin by filing out the diagonals. The main diagonal is identically zero since $m[i,i]=0$, the next diagonal, the case then $i  = j -1$ the $m$ value is given as $p_{i -1} p_i p_j$ 
From this point in, the other values can be filled in using equation 1. Along with the values for $m$ we also fill in the $s$ table denoting the $k$ values between $i$ and $j$ for each entry.
Starting off wit the diagonals we have

$$
m = \
\begin{matrix}
(j\downarrow, i \rightarrow) & 0 & 1 & 2 & 3 & 4 \\
4 & & & & 48& 0\\
3 & & & 60& 0 \\
2 & & 80&0 \\ 
1 & 60& 0 \\
0 & 0 
\end{matrix} 
$$
And the optimal $k$ value along these diagonals is identically the $i$ hence: 
$$
s = \
\begin{matrix}
(j\downarrow, i \rightarrow) & 0 & 1 & 2 & 3 \\
4 & & & & 4 \\
3 & & & 3  \\
2 & & 2 \\ 
1 & 1  \\
 
\end{matrix} 
$$

Now we need to fill i using equation (1)
We have:
$$
 \tag{2} m[2,4] = m[2,2] + m[3, 4] + p_1p_2p_4 = 0 + 48 + 60 = 108
$$

$$
\tag{3} m[1,3] = m[1,2] + m[3, 3] + p_0p_2p_3 = 80 + 0 + 4\cdot 4 \cdot 3 =128 
$$

$$
\tag{4}
m[1,4] = min \begin{cases}m[1,1] + m[2,4] + p_{0}p_{1}p_4 \\ m[1,2] + m[3,4] + p_0p_2p_4 \\ m[1,3] + m[4,4] + p_0p_3p_4 \end{cases}
=  min \begin{cases} 0 + 108 + 4\cdot5\cdot4 \\ 80 + 48 + 4\cdot4\cdot4 \\ 128 + 0 + 4\cdot 3 \cdot4 \end{cases} = min \begin{cases} 188 \\ 192\\ 156 \end{cases} = 156
$$
Note that here the optimal k for equations $(2),(3),(4)$ are respectively $\{2, 2, 3\}$
Carrying on we have 

Note: $p_{-1} = (row)_0 = 3$

$$
\tag{5} m[0,2] = \begin{cases} m[0,0] + m[1,2] + p_{-1}p_{0}p_{2} \\ m[0,1] + m[2, 2] + p_{-1}p_1p_2 \end{cases}= \begin{cases} 0 + 80 + 3\cdot 4\cdot4 \\ 60 + 0 +3 \cdot5 \cdot4  \end{cases} = \begin{cases} 128 \\ 120 \end{cases} = 120
$$
$$
 \tag{6} m[0,3] = min\begin{cases} m[0,0] + m[1,3] + p_{-1}p_{0}p_{3} \\ m[0,1] + m[2,3] + p_{-1}p_1p_3 \\ m[0,2] + m[3,3] + p_{-1} p_2 p_3 \\  \end{cases} 
 = min \begin{cases} 0 + 128 + 3\cdot 4 \cdot 3\\ 60 + 60 + 3\cdot 5 \cdot 3 \\ 120 + 0 + \cdot 3\cdot4 \cdot 3 \end{cases} =
 min \begin{cases} 164 \\ 165 \\ 156 \end{cases} = 156
 $$

$$
\tag{7}
m[0,4] = min \begin{cases} m[0,0] + m[1,4] + p_{-1}p_{0}p_4 \\ m[0,1] + m[2,4] + p_{-1}p_1p_4 \\ m[0,2] + m[3,4] + p_{-1}p_2p_4 \\m[0,3] + m[4,4] + p_{-1}p_3p_4 \end{cases} 
= min \begin{cases} 0 + 156 + 3 \cdot 4 \cdot 4 \\ 60 + 108 +3 \cdot 5 \cdot 4 \\120 + 48 + 3 \cdot 4 \cdot 4  \\ 156 + 0 + 3 \cdot 3 \cdot 4 \end{cases} 
= min \begin{cases} 204 \\ 228 \\ 224\\ 192  \end{cases} = 192
 $$

The optimal $k$'s for equations $(5), (6),(7)$ are respectively $\{1, 2, 3\}$
hence we have


$$
m = \
\begin{matrix}
(j\downarrow, i \rightarrow) & 0 & 1 & 2 & 3 & 4 \\
4 & 192& 156& 120& 48& 0\\
3 & 156& 128& 60& 0 \\
2 & 120& 80&0 \\ 
1 & 60& 0 \\
0 & 0 
\end{matrix} 
$$
and 

$$
s = \
\begin{matrix}
(j\downarrow, i \rightarrow) & 0 & 1 & 2 & 3 \\
4 & 3& 3& 2& 3 \\
3 & 2&2 & 2  \\
2 & 1& 1 \\ 
1 & 0  \\
 
\end{matrix} 
$$
The minimum number of multiplications is hence $192$ and the optimal parenthesization is:

Using the print optimal parens function from the book as a heuristic we have;
$$POP(s, i,j) =\begin{cases} A_i  &\text{ if } i =j \\ \text{'('} + POP(s,i ,s[i,j]) + POP (s,s[i,j]+1,j)\end{cases}$$
Applying this to our $s$ table
$$POP(s,0,  4) =  \text{'('} +  POP(s, 0 , 3) + POP(s, 4, 4)) + \text{')'}$$

$$POP(s,0,3) = \text{'('} + POP(s,0,2) + POP(s,3,3) + \text{')'}$$
$$POP(s,0,2) = \text{'('} + POP(s,0,1) + POP(s,2,2) + \text{')'}$$
We then have: 

$$ POP(s,0,  4) =  \text{'('} + \text{'('} +\text{'('} +\text{'('} + POP(s,0,0) + POP(s,1,1) + $$ $$\text{')'} + POP(s,2,2) + \text{')'} + POP(s,3,3) + \text{')'} + POP(s, 4, 4)) + \text{')'}$$

Which results in:$$((((A_0,A_1)A_2)A_3)A_4)$$

2) Implemented the non recursive matrix chain algorithm from the book in rust. See code in repo for implementation. Testing with the same configuration as in problem 1, we get:

Min number of computations with optimal parenthesization: 192
And that paranthesization is: ((A0(A1(A2A3)))A4)
Running time: 45.417µs

Note that this is not the same results as found in task 1, and that is likely due to the fact that there are some miscalculations in that task.

4) The matrix chain problem exhibits the characteristic of having optimal solutions which is not necessarily built from a sequence of greedy choices. An optimal solution might consist of a non greedy choice which pays out further down the line of the algorithm. As an example, we might have that $(A_0(A_1 A_2))$ have less of an expense than $(A_0A_1(A_2))$, and the first of these would be the greedy choice. However we may have down the line that: $(A_0A_1(A_3A_4))$ is more optimal than $(A_0(A_1A_3)A_4))$ which we would not have reached by choosing greedily. 



## Problem 2 
See code in repo for implementation. 
Implemented two solutions, 1 a solution to the discrete knapsack problem with a recursive bottom up approach. The solution involves checkin the value of every configurable knapsack. The solution works by looping over the set of valuable items, adding an item to the knapsack and calculating the value. Since addition is commutative, if we're at index $i$ for the last item added to the knapsack, we can add item $j$ and store the total value. Later on in the loop when the item $j$ is added to the knapsack and we want to calculate the value for the next items, we do not need to include item $i$ since this has already been checked. 

The fractional case is much simpler. As long as there is space left in the knapsack fill it with the current most valuable "liquid" until either the knapsack reaches its maximum or the all of the valuable liquid is extracted. 

output from test:

``` 
Generated discrete knapsack problem, Valuable items :
ValuableItem { id: 0, size: 2, value: 0 }
ValuableItem { id: 1, size: 5, value: 0 }
ValuableItem { id: 2, size: 7, value: 0 }
ValuableItem { id: 3, size: 5, value: 8 }
ValuableItem { id: 4, size: 3, value: 2 }
ValuableItem { id: 5, size: 6, value: 5 }
ValuableItem { id: 6, size: 6, value: 4 }
ValuableItem { id: 7, size: 3, value: 2 }
Found solution to knapsack problem :
Knapsack { size_left: 0, items: [3, 4, 5], total_value: 15 }
Knapsack { size_left: 0, items: [3, 5, 7], total_value: 15 }
Runtime 173.375µs

 Generated liquid knapsack problem:
ValuableLiquid { volume: 8.30838, value: 0.32589436 }
ValuableLiquid { volume: 1.7982972, value: 9.533528 }
ValuableLiquid { volume: 2.2141361, value: 6.492547 }
ValuableLiquid { volume: 5.196971, value: 2.5793111 }
ValuableLiquid { volume: 6.769079, value: 6.6901875 }
ValuableLiquid { volume: 2.6960206, value: 2.4100125 }
ValuableLiquid { volume: 1.1188173, value: 2.1872807 }
ValuableLiquid { volume: 1.3630486, value: 4.965004 }
Found solution
Knapsack size 6, size of yield 6, total value 22.21201
Duation 4.708µs
```
Note here that the discrete case having a recursive solution is significantly slower than the fractional case. 
## Problem 3
3.1)
Greedy solution. Implemented a greedy solver which gets the wrong sum for the $[1,5,11] \rightarrow 15$ 
problem. Also implemented another greedy strategy from a top down approach, which does find the correct answer of $3 \times 5 = 15$

results

```
 Greedy coin sum solver demo
found solution for target sum 99, using coins [10, 5, 3, 1], with a greedy approach:
coin value: 10, counts: 9
coin value: 5, counts: 1
coin value: 3, counts: 1
coin value: 1, counts: 1
Duration 4.417µs

 Demo coin sum minimzer top down appoach
Found solution to sum target 15, with coins [20, 10, 5, 1]
Found the best coin pouch to be CoinPouch { value: 15, count: 2, coin_stacks: [CoinStack { coin: 10, count: 1 }, CoinStack { coin: 5, count: 1 }] }
```




3.3)
Devise a global optimal solution to the coin sum problem.

Notes.

For a given sum $S$ we want to find the smallest amount of coins from a given set of coins $C$
such that that the sum value of the coins equals the sum $S$

For a set of coins: $C = \{c_1, c_2, c_3, ..., c_n \}$ , and sum $S$. we have:
$$S = r_1c_1 + r_2c_2 + ,..., r_nc_n
$$

where $r_n$ denotes a whole number. 
We want to minimize the number of coins used for the decomposition $R$
$$R =\sum_1^n r_n$$

 We can model the sub sums of the value from the coins as a tree. The optimal substructure is realized as a the simplest tree: that is a tree of one parent node and a leaf for each coin. The optimal substructure comes from the fact that if the parent node is a value (some set of coin values subtracted from the sum value), that is on the minimum path, then it means that the path summing up to it also is the minimum path. Say we are at a node having value S and all it's children are leaf nodes. The value in each leaf corresponds to its cost since that value is the number of one have left from subtracting the coin value. If this was not the case we would not be in a leaf node. Then, from the parent node we can choose the leaf node with the lowest cost. Now we also know the cost of the parent node since this is just adding the coin that we found with the lowest cost. 

We can express this as a recursion:
$$ R(p) = min_{k \in C}\{ 1 + R(c_k) \}$$
That is the value of the parent node in the optimal path is equal to $1 +$ the minimum value of the child nodes. 

### Answer
The structure from the notes is utilized to find the solution, which can be seen in the code in the repo. Solution is implemented with dynamic programming, building up the sub sums of the problem, storing this in a hash table, and building up the minimum path up to the root node. 

#### Results
```
 Demo dynamic coin solver
find optimal sum of value 15, using coins: [1, 5, 11]
Found solution CoinSum { val: 15, coins: [CoinStack { coin: 5, count: 3 }] }
Duration 37.167µs
Test, norwegian coins: [20, 10, 5, 1]
Value 127, found sum CoinSum { val: 127, coins: [CoinStack { coin: 1, count: 2 }, CoinStack { coin: 5, count: 1 }, CoinStack { coin: 20, count: 6 }] }
```

#### Comparison of greedy vs dynamic coins for Norwegian NOK

Test with 10 runs of randomly choosen numbers between 1 and hundred. 
Note here that at least for the dynamic approach, it would be more practical to just run the algorithm once for a larger number and look up the memoized values for the sub sums. However, due to the nature of the implementation, not every sub sum in the memoization is necessarily correct except for the minimal sum path.  
As we can see below for 10 random tests, we can see that the results are the same for both the dynamic and the greedy approach. This might indicate that the NOK coins are greedy coins. Is this proof of this however? no. Also we can see that the dynamic approach is significantly slower than the greedy approach for problems with comparatively large numbers. E.x for the optimal target sum of 245, the greedy approach took 2.333µs while the dynamic approach took 149.25µs.

``` 
 Compare greedy vs dynamic coin solver for norwegian coins: [1, 5, 10, 20], and target sum 41
Greedy approach results:
CoinStack { coin: 20, count: 2 }
CoinStack { coin: 1, count: 1 }
Elapsed time 5.209µs

Dynamic approach: 
CoinStack { coin: 20, count: 2 }
CoinStack { coin: 1, count: 1 }
Time elapsed 40.75µs

 Compare greedy vs dynamic coin solver for norwegian coins: [1, 5, 10, 20], and target sum 17
Greedy approach results:
CoinStack { coin: 10, count: 1 }
CoinStack { coin: 5, count: 1 }
CoinStack { coin: 1, count: 1 }
Elapsed time 2.583µs

Dynamic approach: 
CoinStack { coin: 10, count: 1 }
CoinStack { coin: 5, count: 1 }
CoinStack { coin: 1, count: 2 }
Time elapsed 10.459µs

 Compare greedy vs dynamic coin solver for norwegian coins: [1, 5, 10, 20], and target sum 9
Greedy approach results:
CoinStack { coin: 5, count: 1 }
CoinStack { coin: 1, count: 2 }
Elapsed time 2.209µs

Dynamic approach: 
CoinStack { coin: 5, count: 1 }
CoinStack { coin: 1, count: 4 }
Time elapsed 4.208µs

 Compare greedy vs dynamic coin solver for norwegian coins: [1, 5, 10, 20], and target sum 39
Greedy approach results:
CoinStack { coin: 20, count: 1 }
CoinStack { coin: 10, count: 1 }
CoinStack { coin: 5, count: 1 }
CoinStack { coin: 1, count: 4 }
Elapsed time 2.166µs

Dynamic approach: 
CoinStack { coin: 20, count: 1 }
CoinStack { coin: 10, count: 1 }
CoinStack { coin: 5, count: 1 }
CoinStack { coin: 1, count: 4 }
Time elapsed 26.916µs

 Compare greedy vs dynamic coin solver for norwegian coins: [1, 5, 10, 20], and target sum 245
Greedy approach results:
CoinStack { coin: 20, count: 12 }
CoinStack { coin: 5, count: 1 }
Elapsed time 2.333µs

Dynamic approach: 
CoinStack { coin: 20, count: 12 }
CoinStack { coin: 5, count: 1 }
Time elapsed 149.25µs

 Compare greedy vs dynamic coin solver for norwegian coins: [1, 5, 10, 20], and target sum 179
Greedy approach results:
CoinStack { coin: 20, count: 8 }
CoinStack { coin: 10, count: 1 }
CoinStack { coin: 5, count: 1 }
CoinStack { coin: 1, count: 4 }
Elapsed time 2.292µs

Dynamic approach: 
CoinStack { coin: 20, count: 8 }
CoinStack { coin: 10, count: 1 }
CoinStack { coin: 5, count: 1 }
CoinStack { coin: 1, count: 4 }
Time elapsed 117.792µs

 Compare greedy vs dynamic coin solver for norwegian coins: [1, 5, 10, 20], and target sum 207
Greedy approach results:
CoinStack { coin: 20, count: 10 }
CoinStack { coin: 5, count: 1 }
CoinStack { coin: 1, count: 2 }
Elapsed time 2.667µs

Dynamic approach: 
CoinStack { coin: 20, count: 10 }
CoinStack { coin: 5, count: 1 }
CoinStack { coin: 1, count: 2 }
Time elapsed 137.208µs

 Compare greedy vs dynamic coin solver for norwegian coins: [1, 5, 10, 20], and target sum 158
Greedy approach results:
CoinStack { coin: 20, count: 7 }
CoinStack { coin: 10, count: 1 }
CoinStack { coin: 5, count: 1 }
CoinStack { coin: 1, count: 3 }
Elapsed time 2.208µs

Dynamic approach: 
CoinStack { coin: 20, count: 7 }
CoinStack { coin: 10, count: 1 }
CoinStack { coin: 5, count: 1 }
CoinStack { coin: 1, count: 3 }
Time elapsed 107.792µs

 Compare greedy vs dynamic coin solver for norwegian coins: [1, 5, 10, 20], and target sum 206
Greedy approach results:
CoinStack { coin: 20, count: 10 }
CoinStack { coin: 5, count: 1 }
CoinStack { coin: 1, count: 1 }
Elapsed time 2.292µs

Dynamic approach: 
CoinStack { coin: 20, count: 10 }
CoinStack { coin: 5, count: 1 }
CoinStack { coin: 1, count: 1 }
Time elapsed 136.75µs

 Compare greedy vs dynamic coin solver for norwegian coins: [1, 5, 10, 20], and target sum 124
Greedy approach results:
CoinStack { coin: 20, count: 6 }
CoinStack { coin: 1, count: 4 }
Elapsed time 2.292µs

Dynamic approach: 
CoinStack { coin: 20, count: 6 }
CoinStack { coin: 1, count: 4 }
Time elapsed 79.875µs
```

