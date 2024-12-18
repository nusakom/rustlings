#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    
    // Fix: Don't unwrap on a None value, handle it safely
    if my_option.is_none() {
        println!("The option is None.");
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {my_arr:?}");

    // Fix: Resize doesn't work as intended; simply initialize an empty vector if needed.
    let my_empty_vec: Vec<i32> = Vec::new();
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    
    // Fix: Correct way to swap values
    std::mem::swap(&mut value_a, &mut value_b);

    println!("value a: {value_a}; value b: {value_b}");
}
