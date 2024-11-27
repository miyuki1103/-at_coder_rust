use std::io;

// https://atcoder.jp/contests/adt_easy_20241126_1

// https://atcoder.jp/contests/adt_easy_20241126_1/tasks/abc232_a
pub fn ez1_a(s: String) -> u32{
    let mut first= 0;
    let mut second= 0;
    let mut idx=0;

    let chars: Vec<char>= s.chars().collect();
    for c in chars {
        if idx== 0{
            first= c.to_string().parse().unwrap();
        }else if idx == 2{
            second= c.to_string().parse().unwrap();
        }
        idx+=1;
    }
    first*second
}
