struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("a12345@gmail.com"),
        sign_in_count: 0,
    };
    println!("User1's name is {}", user1.username);

    let mut user2 = User {
        active: true,
        username: String::from("user2"),
        email: String::from("blskc@gmail.com"),
        sign_in_count: 0,
    };
    user2.email = String::from("email@gmail.com");
    println!("User2's email is {}", user2.email);

    let user3 = build_user(String::from("user3@gmail.com"), String::from("user3"));
    println!("User3's active is {}", user3.active);
    println!("User3's username is {}", user3.username);

    let user4 = User {
        username: String::from("user4"),
        email: String::from("user4@gmail.com"),
        ..user1
    };
    println!("User4's active is {}", user4.active);
    println!("User4's email is {}", user4.email);
    println!("User4's sign_in_count is {}", user4.sign_in_count);
    println!("User1's sign_in_count is {}", user1.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        sign_in_count: 1,
        email,
    }
}
