use std::collections::HashMap;

// https://atcoder.jp/contests/abc380/tasks/abc380_a
pub fn q1_a(number: u32) -> String{
    let n_string= number.to_string();
    let v_n: Vec<char>= n_string.chars().collect();
    
    let mut map_one: HashMap<char, u32>= HashMap::new();
    let mut map_two: HashMap<char, u32>= HashMap::new();
    let mut map_three: HashMap<char, u32>= HashMap::new();

    for e_n in v_n {
        match e_n {
            '1' => {
                map_one.insert('1', 1);
            },
            '2' => {
                if *(map_one.get(&'2').unwrap()) == 1 {
                    map_two.insert('2', 2);
                }
                map_one.insert('2', 1);
            },
            '3' => {
                if *(map_one.get(&'3').unwrap()) == 1 {
                    map_two.insert('3', 2);
                }
                if *(map_two.get(&'3').unwrap()) == 2 {
                    map_three.insert('3', 2);
                }
                map_one.insert('2',10);
            },
            _ => { return "No".to_string(); }
        }
    }



    if *(map_one.get(&'1').unwrap()) == 1 
        && *(map_two.get(&'2').unwrap()) == 2 
        && *(map_three.get(&'3').unwrap()) == 3  {
            return "Yes".to_string()
    }

    "No".to_string()
}
