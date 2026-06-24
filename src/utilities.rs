pub fn read_line() -> String{
    use std::io::{stdin, stdout, Write}; // import the stdin and stdout functions from the std::io module
    let mut s: String = String::new(); // mutable s for storing the user input

    let _ = stdout().flush(); // manual flush of the stdout buffer to ensure that the prompt is printed before reading input
    stdin().read_line(&mut s).expect("Did not enter a correct string"); // read a line of input from the user and store it in s, else print an error message

    if let Some('\n') = s.chars().next_back(){  // if the last character of the string is a newline character, remove it
        s.pop(); // pop s
    }
    if let Some('\r')=s.chars().next_back(){ // if the last character of the string is a carriage return character, remove it
        s.pop();
    }

    s.trim().to_string(); // return the trimmed string
    return s;
}

pub fn read_line_as_i32() -> i32{
    let input: String = read_line();
    input.parse::<i32>().unwrap_or_else(|_| {
        println!("Invalid input. Please enter a valid integer.");
        std::process::exit(1);
    });

    return input.parse::<i32>().unwrap();
}

pub fn read_line_as_f32() -> f32{
    let input: String = read_line();
    input.parse::<f32>().unwrap_or_else(|_| {
        println!("Invalid input. Please enter a valid float.");
        std::process::exit(1);
    });

    return input.parse::<f32>().unwrap();
}

pub fn read_line_as_bool() -> bool{
    let input: String = read_line();
    input.parse::<bool>().unwrap_or_else(|_| {
        println!("Invalid input. Please enter a valid boolean (true/false).");
        std::process::exit(1);
    });

    return input.parse::<bool>().unwrap();
}