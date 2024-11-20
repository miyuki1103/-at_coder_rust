use std::io;

// 2024-11-19 1530
pub fn ez_a(v: Vec<u32>) -> String{
    // Create a new mutable string to store the user input
    let mut input = String::new();

    println!("Please enter some text:");

    // Read user input and store it in the `input` variable
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Trim the input to remove any trailing newline or whitespace
    let trimmed_input = input.trim().to_string();

    trimmed_input
}
