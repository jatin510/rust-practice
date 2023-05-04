fn main() {
    println!("Hello, world!");

    // creation
    let a: i32 = 5;

    // mutability
    let mut b: i32 = 5;
    b = 10;

    // shadowing
    let c: i32 = 10;
    let c: i32 = 20;
    println!("c is : {c}");

    // scope
    let d :i32 = 30;
    {
        let d:i32 = 50;
        println!("inner d is : {d}");
    }
    println!("d is : {d}");
}
