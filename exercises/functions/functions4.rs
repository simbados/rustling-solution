// functions4.rs
// Make me compile! Execute `rustlings hint functions4` for hints :)

// This store is having a sale where if the price is an even number, you get
// 10 (money unit) off, but if it's an odd number, it's 3 (money unit) less.

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
    println!("Orginal value is {}", original_price);
}

fn sale_price(mut price: i32) -> i32 {
    let mut sales_price = price;
    if is_even(sales_price) {
        sales_price -= 10
    } else {
        sales_price -= 3
    }
    return sales_price
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
