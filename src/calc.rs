pub fn sum(a: i32, b: i32) -> i32 {
    return a + b; 
}

pub fn subtract(a: i32, b: i32) -> i32 {
    return a - b;
}

pub fn multiply(a: i32, b: i32) -> i32 {
    return a * b;
}

pub fn divide(a: i32, b: i32) -> f32{
    if b != 0{
        return a as f32 / b as f32;
    }
    else{
        return f32::NAN;
    }
}

pub fn my_mod(a: i32, b: i32) -> i32 {
    if b != 0{
        return a % b;
    }
    else{
        return 0; 
    }
}

pub fn power(a: i32, b: i32) -> i32 {
    let mut result: i32 = 1;
    for _ in 0..b {
        result *= a;
    }
    return result;
}

pub fn fact(n: i32) -> i32 {
    if n < 0{
        return 0;
    }
    
    let mut result: i32 = 1;
    for i in 1..=n {
        result *= i;
    }

    return result;
}