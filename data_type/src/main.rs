fn main() {
    // boolean
    let b1: bool = true;

    // unsigned integer
    let i1: u8 = 1;
    let i2: u16 = 2;

    // signed integer
    let i3: i32 = -3;

    // float
    let f1: f32 = 1.9;

    // platform specific integer
    let p1: usize = 1;
    let p2: isize = 2;

    // characters, &str and String
    let c1: char = 'c';
    let s1: &str = "hello";
    let s2: String = String::from("hello");

    // arrays
    let a1: [i32; 5] = [1, 2, 3, 4, 5];

    // tuples
    let t1: (i32, &str) = (1, "hello world");

    let unit: () = ();

    // Type alias
    type age = u8;

    let a1: age = 47;
}
