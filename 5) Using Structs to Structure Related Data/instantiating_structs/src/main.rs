struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("Blake Hourigan"),
        email: String::from("houriganb@pm.me"),
        sign_in_count: 1,
    };

    user1.email = String::from("new_email@urmom.com");

    let user2 = build_user(String::from("Bruh moment"), String::from("email@email.com"));

    println!(
        "{0}, {1}, {2}, {3}",
        user2.active, user2.username, user2.email, user2.sign_in_count
    );

    println!("{0}", user1.email);
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
