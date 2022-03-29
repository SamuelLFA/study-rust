// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn main() {
//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };

//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
// }

// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main() {
//     let home = IpAddr::V4(127, 0, 0, 1);

//     let loopback = IpAddr::V6(String::from("::1"));
// }

// fn main() {
//     let some_number = Some(5);
//     let some_string = Some("a string");

//     let absent_number: Option<i32> = None;

//     let x: i8 = 5;
//     let y: Option<i8> = None;
//     let sum = match y {
//         None => 0,
//         Some(i) => x + i,
//     };
//     println!("{}", sum);
// }

fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
