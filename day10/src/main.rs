use std::fs::File;
use std::io::prelude::*;

const EOL: &str = "\r\n";
fn main() {
    // let mut file = File::open("./input.txt").expect("Unable to open the file");
    let mut file = File::open("./realInput.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");
    let numbers = contents
        .split(EOL)
        .map(|line| {
            let s: Vec<&str> = line.split(" ").collect();
            // println!("{}, {:?}", line, s);
            if s[0] == "noop" {
                return None;
            } else {
                let num = s[1].parse::<i32>().unwrap_or(0);
                return Some(num);
            }
        })
        .collect::<Vec<Option<i32>>>();

    let mut register = 1;
    let mut cycle = 1;
    let mut allCycles = 0;
    // println!("{:?}", numbers);
    let mut screen: String = String::new();

    for (_, num) in numbers.iter().enumerate() {
        draw(cycle, register, &mut screen);
        match num {
            Some(n) => {
                println!("Add {}", n);

                draw(cycle + 1, register, &mut screen);
                // check(cycle + 1, register, &mut allCycles);

                register += n;
                // check(cycle + 2, register, &mut allCycles);
                cycle += 2;
            }
            None => {
                println!("Noop");
                // draw(cycle, register, &mut screen);
                // check(cycle + 1, register, &mut allCycles);
                cycle += 1;
            }
        }
    }
    print!("{}", screen);
    // println!("Sum {}", allCycles);
}

fn draw(cycle: i32, register: i32, screen: &mut String) {
    let hor_pos = (cycle % 40) - 1;
    println!(
        "Drawing cycle {} pixel in position {} , register is {}",
        cycle, hor_pos, register
    );
    if hor_pos == 0 {
        *screen += EOL;
    }
    if register - 1 == hor_pos || register == hor_pos || register + 1 == hor_pos {
        *screen += "#";
    } else {
        *screen += ".";
    }
}

// Tihs was used in part 1
fn check(cycle: i32, register: i32, allCycles: &mut i32) {
    if cycle < 20 {
        return;
    }
    if (cycle - 20) % 40 == 0 {
        println!(
            "Index {}, register {}, strength {} ",
            cycle,
            register,
            cycle * register
        );
        *allCycles += (cycle * register);
    }
}
