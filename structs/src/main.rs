fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
    if rect.width() {
        println!("The rectangle has a nonzero width; it is {}", rect.width);
    }
    println!("{:?}", rect);
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// fn main() {
//     let email = String::from("samuellfa3@gmail.com");
//     let username = String::from("sammurel");
//     let user = build_user(&email, &username);
//     println!("{}\n{}\n{}\n{}\n", user.email, user.username, user.active, user.sign_in_count);
// }

// Struct
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64
// }

// fn build_user(email: &String, username: &String) -> User {
//     return User {
//         email: email.to_string(),
//         username: username.to_string(),
//         active: true,
//         sign_in_count: 1
//     }
// }

// Tuple structs
// fn main() {
//     struct Color(i32, i32, i32);
//     struct Point(i32, i32, i32);

//     let black = Color(10, 0, 0);
//     let origin = Point(0, 0, 0);

//     println!("{}", black.0);
// }
