struct Product {
    name: String,
    price: f32,
    in_stock: bool,
}

impl Product {
    // constructor
    fn new(name: String, price: f32) -> Product {
        Product {
            name,
            price,
            in_stock: true,
        }
    }

    // associated function - don't take self
    fn get_default_sales_tax() -> f32 {
        0.1
    }

    // immutable
    fn calculate_sales_tax(&self) -> f32 {
        self.price * Product::get_default_sales_tax()
    }

    // mutable
    fn set_price(&mut self, price: f32) {
        self.price = price;
    }

    fn buy(self) -> i32 {
        let name = self.name;
        println!("You bought {}", name);
        12323
    }
}
fn main() {
    let mut book = Product::new(String::from("Rust book"), 39.99);

    let sales_tax = book.calculate_sales_tax();
    println!("{}", sales_tax);
    book.set_price(1000.0);
    book.buy();
}
