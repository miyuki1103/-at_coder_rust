
pub fn q1_a(number: u64) -> (u64, u64){
    let chars: Vec<char>= number.to_string()
        .chars()
        .collect();

    let string_one= format!("{}{}{}", chars[1], chars[2], chars[0]);
    let string_two= format!("{}{}{}", chars[2], chars[0], chars[1]);

    let number_1: u64= string_one.parse().unwrap();
    let number_2: u64= string_two.parse().unwrap();

    (number_1,number_2)
}