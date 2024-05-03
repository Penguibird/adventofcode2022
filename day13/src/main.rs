use std::fmt;
use std::fs::File;
use std::io::prelude::*;

const EOL: &str = "\r\n";
const doubleEOL: &str = "\r\n\r\n";

fn main() {
    let mut file = File::open("./realInput.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");

    let mut indexSum = 0;
    let parsed = contents
        .split(doubleEOL)
        // .take(1)
        .map(|par| {
            par.split(EOL)
                .map(parse_packet)
                .map(|(p, _)| {
                    if let Packet::List(v) = p {
                        return v[0].clone();
                    } else {
                        return p;
                    }
                })
                .collect::<Vec<_>>()
        });

    // let x = contents.split(doubleEOL).map(|l| l.split(EOL)).flatten();
    // for (p, l) in parsed.clone().flatten().zip(x) {
    //     println!("Parsed:    ");
    //     println!("{l}");
    //     println!("{p}");
    // }

    // // return;
    // for (i, par) in parsed.enumerate() {
    //     let r = compare_packets(&par[0], &par[1]);
    //     println!("{}", r.unwrap());
    //     match r {
    //         Some(b) => {
    //             if b {
    //                 println!("Adding {i}");
    //                 indexSum += i+1;
    //             }
    //         }
    //         _ => {}
    //     }
    // }
    //
    let mut allpackets = parsed.flatten().collect::<Vec<_>>();
    let a = Packet::List(vec![Packet::List(vec![Packet::Num(2)])]);
    allpackets.push(a);
    let b = Packet::List(vec![Packet::List(vec![Packet::Num(6)])]);
    allpackets.push(b);
    allpackets.sort_by(|a, b| {
        let res = compare_packets(a, b);
        match res {
            Some(false) => std::cmp::Ordering::Greater,
            Some(true) => std::cmp::Ordering::Less,
            None => std::cmp::Ordering::Equal,
        }
    });
    // allpackets.iter().for_each(|p| println!("{p}"));
    let a = allpackets
        .iter()
        .position(|p| p.to_string() == "[[2]]")
        .unwrap()+1;

    let b = allpackets
        .iter()
        .position(|p| p.to_string() == "[[6]]")
        .unwrap()+1;
    println!("Result is {}", a * b);
}

#[derive(Clone)]
enum Packet {
    Num(i32),
    List(Vec<Packet>),
}
impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_packet(self, f)
    }
}
fn format_packet(p: &Packet, f: &mut fmt::Formatter) -> fmt::Result {
    match p {
        Packet::Num(n) => {
            write!(f, "{n}")
        }
        Packet::List(l) => {
            write!(f, "[");
            for (i, p) in l.iter().enumerate() {
                format_packet(p, f);
                if i != l.len() - 1 {
                    write!(f, ",");
                }
            }
            write!(f, "]")
        }
    }
}
fn compare_packets(pa: &Packet, pb: &Packet) -> Option<bool> {
    println!("Comparing  {pa}{EOL} to packet {pb}");
    let r = match pa {
        Packet::Num(na) => match pb {
            Packet::Num(nb) => {
                if na == nb {
                    None
                } else {
                    Some(na < nb)
                }
            }
            Packet::List(lb) => compare_packets(&Packet::List(vec![pa.clone()]), pb),
        },
        Packet::List(la) => match pb {
            Packet::Num(nb) => compare_packets(pa, &Packet::List(vec![pb.clone()])),
            Packet::List(lb) => {
                let maxI = std::cmp::max(la.len(), lb.len());
                for i in 0..maxI {
                    let la_packet = la.get(i);
                    let lb_packet = lb.get(i);
                    if let None = la_packet {
                        return Some(true);
                    }
                    if let None = lb_packet {
                        return Some(false);
                    }

                    let res = compare_packets(la_packet.unwrap(), lb_packet.unwrap());
                    match res {
                        None => {}
                        Some(b) => return Some(b),
                    }
                }
                return None;
            }
        },
    };
    match r {
        None => {
            println!("Comparing {pa} to {pb}. Result is None")
        }
        Some(b) => {
            println!("Comparing {pa} to {pb}. Result is {b}")
        }
    };

    return r;
}

fn parse_packet(s: &str) -> (Packet, usize) {
    // if s.len() == 1 {
    //     return Packet::Num(s.parse::<i32>().unwrap());
    // }
    let mut list: Vec<Packet> = Vec::new();
    // for (i, char) in s.chars().enumerate() {
    // println!("Going in");
    let mut i = 0;
    let mut word = String::new();
    while i < s.len() {
        let char = s.chars().collect::<Vec<_>>()[i];
        // println!("Checking index {i} character {char}");
        let mut flush_word = || {
            if let Ok(n) = word.parse::<i32>() {
                list.push(Packet::Num(n));
            }
            word = String::new();
        };
        if char == ',' {
            // list.push(.unwrap()));
            flush_word();
        } else if char == '[' {
            let (p, di) = parse_packet(&s[i + 1..]);
            // println!("Returning");
            i += di + 1;
            list.push(p);
        } else if char == ']' {
            flush_word();
            return (Packet::List(list), i);
        } else {
            // println!("Character {char}");
            word += &char.to_string();
        }

        i += 1;
    }
    return (Packet::List(list), s.len());
}
