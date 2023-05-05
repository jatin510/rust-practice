fn main() {
    let mut s1 = String::from("Rust");

    // borrowing
    let r1 = &s1;
    print_string(r1);

    // references
    let r2 = &mut s1;
    add_to_string(r2);
    // shadowing
    // let s1 = add_to_string(s1);
    println!("s1 = {}", s1);

    let s2 = generate_string();
    println!("s2 = {}", s2);
}

fn print_string(p1: &String) {
    println!("p1 = {}", p1);
}

fn add_to_string(p1: &mut String) {
    p1.push_str(" is awesome");
}

fn generate_string() -> String {
    let s = String::from("I am generated string");
    s
}
