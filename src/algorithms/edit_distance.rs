use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash)]
enum Operations {
    MatchSub,
    Insert,
    Delete,
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
