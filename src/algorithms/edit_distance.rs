use std::{cmp::max, collections::HashMap};
use super::super::data_structures::array_2d::Array2D;

#[derive(Eq, PartialEq, Hash)]
enum Operations {
    MatchSub,
    Insert,
    Delete,
    Length,
}

fn match_cost(s: &str, t:&str) -> usize {
    if s[0..1] == t[0..1] {
        println!("Matched {:?}", s);
        0
    } else {
        1
    }
}

// From Skiena's Algorithm Design Manual page 316
// Returns the edit distance between string s and t starting 
// from the beginning (index 0) and ending at index i (inclusive)
// Edit distance is defined as the number of substitutions, insertions, or deletions
// to the second string (t) to make it match the first string (s)
pub fn string_compare_r(s: &str, t: &str) -> usize {

    println!("s {:?} t {:?}", s, t);

    if s.len() == 0 {
       // we have reached reached the beginning of the first string s, starting from the end
       // the edit distance is deleting any remaining characters in t
       return t.len();
    }

    if t.len() == 0 {
        // we have reached the beginning of the second string t , starting from the end 
        // the edit distance is inserting any remaining characters in s 
        return s.len();
    }

    let mut opts = HashMap::<&Operations, usize>::new();
 
    let s_len = s.len();
    let t_len = t.len();
    let s_len_less = s_len - 1;
    let t_len_less = t_len - 1;

     // match or substitute in t
    let match_dist = string_compare_r(&s[..s_len_less], &t[..t_len_less]) + match_cost(&s[s_len_less..], &t[t_len_less..]);
    opts.insert(&Operations::MatchSub, match_dist);

     // insert into t
    let insert_dist = string_compare_r(&s, &t[..t_len_less]) + 1;
    opts.insert(&Operations::Insert, insert_dist);

    // delete from t
    let delete_dist = string_compare_r(&s[..s_len_less], &t) + 1;
    opts.insert(&Operations::Delete, delete_dist);
                                                                        //
    match opts.values().min() {
        Some(x) => *x,
        None => 0,
    }
}

struct EditDistance {
    cost: usize,
    prev_op: Operations,
}

impl Clone for EditDistance {
    fn clone(&self) -> Self {
        Self { cost: self.cost.clone(), prev_op: self.prev_op }
    }
}

fn string_compare_grid_helper(s: &str, t: &str, grid: &mut Array2D<EditDistance>) -> usize {
    let i = s.len() - 1;
    let j = t.len() - 1;

    let mut cost: [usize; Operations::Length as usize];

    let subst_cost = match s[i..i] == t[j..j] {
        true => 1,
        false => 0,
    };

    cost[Operations::MatchSub as usize] = grid.get(i-1, j-1).unwrap().cost + subst_cost;
    cost[Operations::Insert as usize] = grid.get(i-1, j).unwrap().cost + 1;
    cost[Operations::Delete as usize] = grid.get(i, j-1).unwrap().cost + 1;

    let mut min_cost = cost[Operations::MatchSub as usize];
    let mut min_prev_op = Operations::MatchSub;

    let ops = vec![Operations::Insert, Operations::Delete];
    for op in ops.iter() {
        if cost[*op as usize] < min_cost {
            min_cost = cost[*op as usize];
            min_prev_op = *op;
        }
    }

    grid.set(i,j, EditDistance { cost: min_cost, prev_op: min_prev_op });

    min_cost
}

// bootstrap call to dynamic programming version of edit distance
// Parameters:
//   s - first string to compare 
//   t - second string to compare
// Returns
//   number of edits to transform s into t
//   (substitutions, insertions, deletions)
fn string_compare_grid(s: &str, t: &str) -> usize {
    
    // Initialize grid to maximum value (one more than the maximum of either s or t)
    let max_len = s.len() + t.len() + 1;

    let DEFAULT = EditDistance {
        cost: max_len + 1,
        prev_op: Operations::MatchSub,
    };

    // Row-major grid to store subproblems of edit distance between
    // s and t starting at beginning (index 0) and up to index i (row i)
    // and up to index j (row j)
    let mut grid: Array2D<EditDistance> = Array2D::new_with_default(s.len() + 1, t.len() + 1, DEFAULT);

    for col in 0..s.len()+1 {
        grid.set(0,col, EditDistance { cost: col, prev_op: Operations::Insert } );
    }

    string_compare_grid_helper(s, t, &mut grid)
}


