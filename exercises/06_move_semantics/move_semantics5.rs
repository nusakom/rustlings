fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

fn string_uppercase(data: String) {
    let data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    // We pass a reference to `get_char` since it doesn't need to own `data`
    let last_char = get_char(&data);

    // `data` can no longer be used after this point because it has been moved
    // into `string_uppercase`. If you need to use `data` after this, you should clone it first.
    string_uppercase(data);

    // If you need to use `last_char` after calling `string_uppercase`, you can do so here.
    println!("The last character is: {}", last_char);
}
