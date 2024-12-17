fn calculate_price_of_apples(quantity: i32) -> i32 {
    if quantity > 40 {
        quantity // 每个苹果只需 1 rustbuck
    } else {
        quantity * 2 // 每个苹果需要 2 rustbucks
    }
}

fn main() {
    // 可以在此处进行测试
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);  // 35 apples, 2 rustbucks each => 70 rustbucks
        assert_eq!(calculate_price_of_apples(40), 80);  // 40 apples, 2 rustbucks each => 80 rustbucks
        assert_eq!(calculate_price_of_apples(41), 41);  // 41 apples, 1 rustbuck each => 41 rustbucks
        assert_eq!(calculate_price_of_apples(65), 65);  // 65 apples, 1 rustbuck each => 65 rustbucks
    }
}
