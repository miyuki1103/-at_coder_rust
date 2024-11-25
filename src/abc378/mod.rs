use std::collections::HashMap;

// https://atcoder.jp/contests/abc378/tasks/abc378_a
pub fn q1_a(numbers: Vec<i32>) -> i32{
    let mut hashmap : HashMap<i32, i32>= HashMap::new();
    let mut hashmap_full : HashMap<i32, i32>= HashMap::new();
    let mut cnt: i32= 0;
    for e_n in numbers {
        if cnt==1 && hashmap_full.contains_key(&e_n) {
            return cnt;
        }
        if hashmap.contains_key(&e_n){
            hashmap_full.insert(e_n, 2);
            cnt+=1;
        }
        hashmap.insert(e_n, 1);
    }
    cnt
}
