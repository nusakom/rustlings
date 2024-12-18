struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let author = "George Orwell";
    let title = "1984";
    let book = Book {
        author,
        title,
    };

    println!("{} by {}", book.title, book.author);
}