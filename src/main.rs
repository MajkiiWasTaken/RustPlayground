// when user builds a new cargo via cargo new <project_name>, this is the default main.rs file
mod calc; // linkage to the calc.rs file in the same directory


fn main() {
    // let mut m: i32 = 5;  // if user wants to change the value of a var mut is needed as a declaration param
    // println!("The value of m is: {m}"); // println! is a macro thats why there is a ! after the name of the macro
    //
    // m = 6; // <- such as here where the user wants to change the value of m 
    
    println!("{}", calc::sum(5, 3));
    println!("{}", calc::subtract(5, 3));
    println!("{}", calc::multiply(5, 3));
    println!("{}", calc::divide(5, 0));
    println!("{}", calc::my_mod(5, 3));
    println!("{}", calc::power(5, 3));
    println!("{}", calc::fact(5));

}