use std::fmt;
use std::fs::File;
use std::io::prelude::*;

const EOL: &str = "\r\n";
const doubleEOL: &str = "\r\n\r\n";

fn main() {
    let mut file = File::open("./real_input.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");

    let mut parsed = contents
        .split(EOL)
        .map(|n| n.parse::<i32>().unwrap())
        .map(|n| (n, false))
        .collect::<Vec<_>>();
    println!("{}", parsed.len());

    let parsed_len = parsed.len() as i32 -1;

    let mut i = 0;
    while i < parsed_len {
        // println!(
        //     "{:?}",
        //     parsed.clone().iter().map(|a| a.0).collect::<Vec<_>>()
        // );
        let (mut number, mut visited) = parsed[i as usize].clone();
        if visited {
            i += 1;
            continue;
        }
        // println!("Moving {number}");
        // if number + i < 0 {
        //     number -= 1;
        // }
        if number > 0 && number + i > parsed_len {
            number += number / parsed_len;
        }
        let new_i = ((positivise(number + i, parsed_len)) % parsed_len) as usize;

        let mut r = parsed.remove(i as usize);
        r.1 = true;
        parsed.insert(new_i, r);

        if new_i > i as usize {
        } else {
            i += 1;
        }
    }

    let zero_i = parsed.iter().position(|x| x.0 == 0).unwrap();
    println!(
        "{:?}",
        parsed.clone().iter().map(|a| a.0).collect::<Vec<_>>()
    );
    println!("Zero i {zero_i}");
    let res = vec![
        get_at_wrap(&parsed, zero_i + 1000),
        get_at_wrap(&parsed, zero_i + 2000),
        get_at_wrap(&parsed, zero_i + 3000),
    ]
    .iter()
    .map(|x| x.0)
    .map(|s| {
        println!("{s}");
        s
    })
    .sum::<i32>();
    println!("Result {}", res);
}

fn get_at_wrap<T>(v: &Vec<T>, i: usize) -> &T {
    let fin_i = i % (v.len()-1);
    return &v[fin_i];
}
fn positivise(_n: i32, m: i32) -> i32 {
    let mut n = _n;
    while n < 0 {
        n += m;
    }
    n
}
