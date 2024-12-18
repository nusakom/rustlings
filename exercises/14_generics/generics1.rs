fn main() {
    // Annotate the type of the vector as `Vec<i32>`.
    let mut numbers: Vec<i32> = Vec::new();

    // Don't change the lines below.
    let n1: u8 = 42;
    numbers.push(n1.into()); // `n1.into()` converts `n1` to `i32`.
    let n2: i8 = -1;
    numbers.push(n2.into()); // `n2.into()` converts `n2` to `i32`.

    println!("{numbers:?}");
}
