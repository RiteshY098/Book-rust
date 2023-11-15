use std::collections::HashMap;

struct Book {
    id: i32,
    title: String,
    author: String,
    year: i32,
}

fn create_book(id: i32, title: String, author: String, year: i32) -> Book {
    Book {
        id,
        title,
        author,
        year,
    }
}

fn read_book(id: i32, books: &HashMap<i32, Book>) -> Option<&Book> {
    books.get(&id)
}

fn update_book(id: i32, new_title: String, new_author: String, new_year: i32, books: &mut HashMap<i32, Book>) {
    if let Some(book) = books.get_mut(&id) {
        book.title = new_title;
        book.author = new_author;
        book.year = new_year;
    }
}

fn delete_book(id: i32, books: &mut HashMap<i32, Book>) {
    books.remove(&id);
}

fn main() {
    let mut books = HashMap::new();

    // Create book
    let book1 = create_book(1, "The Lord of the Rings".to_string(), "J.R.R. Tolkien".to_string(), 1954);
    books.insert(book1.id, book1);

    // Read book
    let book = read_book(1, &books);
    if let Some(book) = book {
        println!("Book title: {}", book.title);
        println!("Book author: {}", book.author);
        println!("Book year: {}", book.year);
    } else {
        println!("Book with ID {} not found", 1);
    }

    // Update book
    update_book(1, "The Hobbit".to_string(), "J.R.R. Tolkien".to_string(), 1937, &mut books);

    // Read book again
    let book = read_book(1, &books);
    if let Some(book) = book {
        println!("Book title: {}", book.title);
        println!("Book author: {}", book.author);
        println!("Book year: {}", book.year);
    } else {
        println!("Book with ID {} not found", 1);
    }

    // Delete book
    delete_book(1, &mut books);

    // Read book again (should not exist anymore)
    let book = read_book(1, &books);
    if let Some(book) = book {
        println!("Book title: {}", book.title);
        println!("Book author: {}", book.author);
        println!("Book year: {}", book.year);
    } else {
        println!("Book with ID {} not found", 1);
    }
}
