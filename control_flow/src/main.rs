fn main() {
    println!("Hello, world!");

    let a: i32 = 5;

    if a > 5 {
        println!("a is greater than 5");
    } else if a == 5 {
        println!("a is equal to 5");
    } else {
        println!("a is less than 5");
    }

    let b: f32 = if a > 5 { 1.9 } else { 0.9 };

    let _ = b;

    loop {
        println!("loop forever");
        break;
    }

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    for element in a {
        println!("the value is: {}", element);
    }
}
