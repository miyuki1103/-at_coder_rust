use rand::prelude::*;

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

pub fn ez2_a(n: i32, c: i32, times: Vec<i32>) -> i32{
    1
}

pub fn d27_ez3_a(a: i16, b: i16) -> i16{
    let mut rng = rand::thread_rng();
    let number= {
        let mut random_number= 0;
        while random_number != a+b {
            random_number= rand::thread_rng().gen_range(0..9);
            
            return random_number;
        }

        return random_number;
    };

    number
}

// https://atcoder.jp/contests/adt_easy_20241127_2/tasks/abc237_a
pub fn d0241127_2_ez_b(n_str: String) -> bool{
    let integer_n= n_str.parse::<i32>().unwrap_or(-99);

    if integer_n == -99 {
        return false;
    }

    true
}
