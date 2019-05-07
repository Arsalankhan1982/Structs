fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    let mut user1 = User {
        email: String::from("arsalankhan1982@gmail.com"),
        username: String::from("arsalankhan1982"),
        active: true,
        sign_in_count: 2,
    };

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 10,
        }
    }
    let user2 = build_user(
        String::from("Arsalankhan1982@gmail.com"),
        String::from("Arsalankhan1982"),
    );
    println!("{}", user1.email);
    user1.email = String::from("arsalankhan1982@outlook.com");
    println!(
        "{} ,{} ,{} ,{}",
        user1.email, user1.username, user1.sign_in_count, user1.active
    );
    println!(
        "{}, {}, {}, {}",
        user2.email, user2.username, user2.active, user2.sign_in_count
    );
}
