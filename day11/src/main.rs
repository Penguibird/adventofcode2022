use eval::{eval, to_value, Error, Value};
use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;

const EOL: &str = "\r\n";
const doubleEOL: &str = "\r\n\r\n";

#[derive(Debug, Clone)]
struct Monkey<T> {
    items: Vec<i64>,
    operation: T,
    divisible_by: i64,
    target_if_true: i64,
    target_if_false: i64,
    total_number_of_inspections: i64,
}

fn main() {
    // let mut file = File::open("./input.txt").expect("Unable to open the file");
    let mut file = File::open("./realInput.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");

    let mut monkeys = contents
        .split(doubleEOL)
        .map(|paragraph| {
            let lines: Vec<&str> = paragraph.split(EOL).collect();

            let items = lines[1].split(": ").collect::<Vec<&str>>()[1]
                .split(", ")
                .map(|x| x.parse::<i64>())
                .filter_map(|x| match x {
                    Ok(n) => Some(n),
                    _ => None,
                })
                .collect::<Vec<i64>>();

            let operation = lines[2].split("new = ").collect::<Vec<&str>>()[1];
            let operation = |n: i64| {
                // println!("Operation  value {}", n);
                eval(&operation.replace("old", &n.to_string()))
            };

            let divisible_by = lines[3].split("by ").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap_or(1);
            let target_if_true = lines[4].split("monkey ").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap();
            let target_if_false = lines[5].split("monkey ").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap();
            return Monkey {
                divisible_by,
                items,
                operation,
                target_if_false,
                target_if_true,
                total_number_of_inspections: 0,
            };
        })
        .collect::<Vec<Monkey<_>>>();
let supermodulo = monkeys.iter().map(|m| m.divisible_by).reduce(|a,b| a*b).unwrap();
    for i in 0..10000 {
        for monkeyI in 0..monkeys.len() {
            let monkey = monkeys[monkeyI].clone();
            for item in monkey.items.iter() {
                monkeys[monkeyI].total_number_of_inspections += 1;
                let i = (monkey.operation)(*item)
                    .expect("Couldt evaluate expression")
                    .as_i64()
                    .unwrap() as i64;
                // let i = (i as f64 / 3 as f64).floor() as i64;
                let i = i % supermodulo;

                let mut target: usize;
                if i % monkey.divisible_by == 0 {
                    target = monkey.target_if_true as usize;
                } else {
                    target = monkey.target_if_false as usize;
                }
                let k = monkeys[target].divisible_by;
                monkeys[target].items.push(i);
            }
            monkeys[monkeyI].items.clear();
        }
    }
    let mut sorted = monkeys
        .iter()
        .map(|m| m.total_number_of_inspections)
        .collect::<Vec<_>>();
    sorted.sort();
    let fin = sorted
        .into_iter()
        .rev()
        .take(2)
        .reduce(|x, a| x * a)
        .unwrap();
    println!("{}", fin);
}
