fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    let _user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The value of the rectangle is {} square pixels.",
        rectangle_area(&rect1)
    );

    println!("rect1 is {rect1:?}");

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn rectangle_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}