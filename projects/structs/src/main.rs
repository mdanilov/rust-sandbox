struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        sign_in_count: 1,
        active: true,
    };

    user1.email = String::from("anotheremail@example.com");
    println!(
        "{} {} {} {}",
        user1.email, user1.username, user1.sign_in_count, user1.active
    );

    build_user(
        String::from("another@example.com"),
        String::from("anotherusername567"),
    );

    let _user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1 // struct update syntax
    };

    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // field init shorthand
        username,
        active: true,
        sign_in_count: 1,
    }
}
