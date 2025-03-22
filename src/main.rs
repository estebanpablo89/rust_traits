mod basket;

use basket::Basket;

fn main() {
    let b1 = Basket::new(String::from("apple"));

    let b2 = Basket::new(23);

    let b3 = Basket::new(true);
}
