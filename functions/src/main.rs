fn main() {
    let z: i32 = my_function(4);
    println!("The value of z is: {}", z);
}

fn my_function(x: i32) -> i32 {
    println!("The value of x is: {}", x);
    let y: i32 = 5;
    return y;
}
