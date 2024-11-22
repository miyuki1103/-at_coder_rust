pub fn q1_a(n: u32, s: String) -> String{
    if n == 1 && s == "/".to_string(){
        return "Yes".to_string();
    }
    if n % 2 != 0{
        return "No".to_string();
    }

    let mut chars: Vec<char>= s.chars().collect();
    
    for _ in 0..(chars.len() / 2) {
        let poped= chars.pop().unwrap();
        if poped != '2' {
            return "No".to_string();
        }
    }

    println!("{:?}", chars);
    chars.pop();
   
   while let Some(vavle)= chars.pop() {
        if vavle != '1' {
            return "No".to_string();
        }
   } 

    "Yes".to_string()
}
