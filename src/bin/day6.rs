fn main() {
    struct User {
        username: String,
        email: String,
        signed_in: bool,
        password: String
    }

    let user1 = User {
        username: String::from("minhozinho"),
        email: String::from("minho@gmail.com"),
        signed_in: true,
        password: String::from("minho123")
    };

    let user2 = User {
        username: String::from("zinho"),
        email: String::from("zinho@proton.me"),
        signed_in: false,
        password: String::from("123zinho")
    };

    println!("Username: {} email: {}", user1.username, user1.email);

    println!("passwords of user1 and user 2 are {} and {}", user1.password, user2.password);

    struct Color(i32, i32, i32);

    fn color() {
        let black = Color(15,56,256);
        println!("Color: {}, {}, {}", black.0, black.1, black.2)
    }

    color()
}