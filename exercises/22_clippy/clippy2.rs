fn main() {
    let mut res = 42;
    let option = Some(12);
    
    // Fix the Clippy lint by using `if let`
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}
