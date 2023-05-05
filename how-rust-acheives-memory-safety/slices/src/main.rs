fn main() {
    // slices

    let tweet = String::from("Hello, World !");

    let trimmed_tweet: &str = trim_tweet(&tweet);

    let tweet2: &str = "Hello, World !";

    let trimmed_tweet2: &str = trim_tweet(tweet2);

    println!("{trimmed_tweet}");
    println!("{trimmed_tweet2}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a_slice = &a[..3];
    println!("{:?}", a_slice);
}

fn trim_tweet(tweet: &str) -> &str {
    &tweet[..5]
}
