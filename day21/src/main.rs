use std::collections::HashMap;
use std::io::prelude::*;
use std::ops::Div;
use std::{fs::File, ops::Mul};

const EOL: &str = "\r\n";
const doubleEOL: &str = "\r\n\r\n";

fn main() {
    let mut file = File::open("./input.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");

    let monkeys = contents
        .split(EOL)
        .map(|line| {
            let mut split = line.split(": ");
            let key = split.next().unwrap();
            let sec = split.next().unwrap();
            if sec.contains(" ") {
                let s = sec.split(" ").collect::<Vec<_>>();
                let o = (
                    s[0],
                    s[2],
                    match s[1] {
                        "+" => Plus,
                        "-" => Minus,
                        "*" => Multiply,
                        "/" => Divide,
                        _ => panic!("Error"),
                    },
                );
                return (key, Operation(o));
            } else {
                let n = sec.parse::<BigInt>().unwrap();
                return (key, Number(n));
            }
        })
        .collect::<HashMap<_, _>>();

    // for m in monkeys {
    //     println!("{m:?}");
    // }

    // let res = calculate_monkey("root", &monkeys);
    // println!("{res}");

    let x = traverse_search_for_hmn("root", &monkeys).unwrap();
    let mut parents = x.into_iter().rev().collect::<Vec<_>>();

    parents.pop();
    parents.insert(0, "root");
    println!("{parents:?}");
    let x = get_parent_target_number("humn", &monkeys, &mut parents);

    println!("{x}");
}

fn get_parent_target_number(
    mon_key: &str,
    monkeys: &HashMap<&str, Monkey>,
    parents: &mut Vec<&str>,
) -> BigInt {
    let parent = parents.pop().unwrap_or_else(|| panic!("{mon_key}"));
    if parent == "root" {
        let m = monkeys.get(parent).unwrap();
        match m {
            Number(_) => panic!("Error"),
            Operation((m1, m2, _)) => {
                let mut other_m = m1;
                if (m2 != &mon_key) {
                    other_m = m2;
                }
                return calculate_monkey(&other_m, monkeys);
            }
        }
    }

    match monkeys.get(parent).unwrap() {
        Number(_) => panic!("{parent}"),
        Operation((m1, m2, op)) => {
            let mut m = m1;
            if *m2 != mon_key {
                m = m2;
            }
            let m = &calculate_monkey(m, monkeys);
            let res = &get_parent_target_number(parent, monkeys, parents);
            let x =  match op {
                Plus => res - m,
                Minus => res + m,
                Divide => res * m,
                Multiply => res / m,
            };
            if match op {
                Plus => x +m == res,
                Multiply => x *m == res,
                Divide => x /m == res || m / x == res,
                Minus => x -m == res || m -x == res,
            } {

            } else {
                
            }
            println!("Monkey {} with parent {} has to be number {} operation {:?}, result {}, m {}", mon_key, parent, x, op, res, m );
            
            return  x;
        }
    }
}

fn traverse_search_for_hmn<'a>(
    mon_key: &'a str,
    monkeys: &'a HashMap<&str, Monkey>,
) -> Option<Vec<&'a str>> {
    if mon_key == "humn" {
        return Some(Vec::new());
    }
    match monkeys.get(mon_key).unwrap() {
        Number(_) => None,
        Operation((m1, m2, _)) => {
            let a = traverse_search_for_hmn(m1, monkeys);
            let b = traverse_search_for_hmn(m2, monkeys);
            if let Some(mut o) = a {
                o.push(m1);
                return Some(o);
            }
            if let Some(mut o) = b {
                o.push(m2);
                return Some(o);
            }
            return None;
        }
    }
}

fn calculate_monkey(mon_key: &str, monkeys: &HashMap<&str, Monkey>) -> BigInt {
    match monkeys.get(mon_key).unwrap() {
        Number(n) => n.clone(),
        Operation((m1, m2, op)) => match op {
            Plus => calculate_monkey(m1, monkeys) + calculate_monkey(m2, monkeys),
            Minus => calculate_monkey(m1, monkeys) - calculate_monkey(m2, monkeys),
            Divide => calculate_monkey(m1, monkeys) / calculate_monkey(m2, monkeys),
            Multiply => calculate_monkey(m1, monkeys) * calculate_monkey(m2, monkeys),
        },
    }
}

#[derive(Debug)]
enum Monkey<'a> {
    Number(BigInt),
    Operation((&'a str, &'a str, Op)),
}
use Monkey::{Number, Operation};

#[derive(Debug)]
enum Op {
    Plus,
    Minus,
    Multiply,
    Divide,
}
use num::{bigint, BigInt};
use Op::{Divide, Minus, Multiply, Plus};
