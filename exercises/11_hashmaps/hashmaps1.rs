use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // Declare the hash map.
    let mut basket = HashMap::new();

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);

    // Put more fruits in your basket.
    basket.insert(String::from("apple"), 1); // 1 apple
    basket.insert(String::from("mango"), 2); // 2 mangoes

    basket
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
