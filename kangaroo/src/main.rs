use std::io;

// Inputs example:
// 0 3 4 2 -> YES
// 0 2 5 3 -> NO

fn read_values() -> (u64, u64, u64, u64) {
    let mut line = String::new();
    
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let mut inputs = line.split(" ");
    let x1: u64 = inputs.next().unwrap().parse::<u64>().unwrap();
    let v1: u64 = inputs.next().unwrap().parse::<u64>().unwrap();
    let x2: u64 = inputs.next().unwrap().parse::<u64>().unwrap();
    let v2: u64 = inputs.next().unwrap().parse::<u64>().unwrap();
    (x1, v1, x2, v2)
}

fn main() {
    let (mut x1, v1, mut x2, v2) = read_values();
    if v1 > v2 {
        while x1 <= x2 {
            if x1 == x2 {
                println!("YES");
                return;
            } else {
                x1 += v1;
                x2 += v2;
            }
        }
        println!("NO");
    }

    else if v2 > v1 {
        while x2 <= x1 {
            if x1 == x2 {
                println!("YES");
                return;
            } else {
                x1 += v1;
                x2 += v2;
            }
        }
        println!("NO")
    }

    else {
        if x1 == x2 {
            println!("YES");
        }
        else {
            println!("NO");
        }
    }
}
