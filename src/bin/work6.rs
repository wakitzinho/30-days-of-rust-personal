fn main() {
    struct Book {
        title: String,
        author: String,
        pages: i32,
    }

    let book1 = Book {
        title: String::from("1984"),
        author: String::from("George Orwell"),
        pages: i32::from(250)
    };

    let book2 = Book {
        title: String::from("48 laws of power"),
        author: String::from("minhosinhgo"),
        pages: i32::from(48)
    };

    println!("book 1 title:  {} book 1 pages: {}", book1.title, book1.pages);
    println!("book 2 author: {} ", book2.author);



}