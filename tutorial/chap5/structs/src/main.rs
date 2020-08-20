#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

/***
 * Tuple structs are useful when you want to give the whole tuple a name and
 * make the tuple be a different type from other tuples, and naming each field
 * as in a regular struct would be verbose or redundant.
 */
#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: false,
        sign_in_count: 10,
    };

    user2.email = String::from("anotheremail@example.com");

    let user3 = build_user(String::from("hoge@example.com"), String::from("taro"));
    println!("The user3 is: {:#?}", user3);

    let user4 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user5 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,      // email: email,
        username,   // username: username,
        active: true,
        sign_in_count: 1,
    }
}