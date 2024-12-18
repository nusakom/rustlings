use std::num::ParseIntError;

fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // Handle the Result returned by total_cost
    match total_cost(pretend_user_input) {
        Ok(cost) => {
            if cost > tokens {
                println!("You can't afford that many!");
            } else {
                tokens -= cost;
                println!("You now have {tokens} tokens.");
            }
        }
        Err(e) => {
            // Handle the error, for example by printing an error message
            eprintln!("Error parsing item quantity: {e}");
        }
    }
}
