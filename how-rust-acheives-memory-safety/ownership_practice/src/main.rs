fn main() {
    println!("Hello, world!");

    let s1 = String::from("Rust");
    // let s2 = s1;
    let s3 = generate_string();
    let s4 = add_to_string(s3);

    print_string(s1.clone());

    println!("s1 is {}", s1);
    // println!("s3 is {}", s3);
    println!("s4 is {}", s4);

    let x = 40;
    print_integer(x);
    println!("x is {}", x);
}

fn print_string(p1: String) {
    println!("p1 is {}", p1);
}

fn generate_string() -> String {
    String::from("Hello, world!")
}

fn add_to_string(mut s: String) -> String {
    s.push_str(" is awesome!");

    return s;
}

fn print_integer(i: i32) {
    println!("i is {}", i);
}
