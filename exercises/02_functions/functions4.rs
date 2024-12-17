fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// 修复函数签名，指定返回类型为 i64
fn sale_price(price: i64) -> i64 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}
