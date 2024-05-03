use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug, Display};
use std::fs::File;
use std::io::prelude::*;
use std::iter::Map;

const EOL: &str = "\r\n";
const doubleEOL: &str = "\r\n\r\n";

fn main() {
    let mut file = File::open("./real_input.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");

    let mut rows: HashMap<i32, Vec<Range>> = HashMap::new();
    let mut beacons: HashMap<i32, HashSet<(i32, i32)>> = HashMap::new();

    contents.split(EOL).for_each(|line| {
        let trimmed = line
            .replace("Sensor at x=", "")
            .replace("y=", "")
            .replace(": closest beacon is at x=", ", ");
        // println!("{trimmed}");
        let arr = trimmed
            .split(", ")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let sensor_pos = (arr[0], arr[1]);
        let beacon_pos = (arr[2], arr[3]);

        let dist = (beacon_pos.0 - sensor_pos.0).abs() + (beacon_pos.1 - sensor_pos.1).abs();

        let x = beacons.get_mut(&beacon_pos.1);
        match x {
            None => {
                let mut h = HashSet::new();
                h.insert(beacon_pos);
                beacons.insert(beacon_pos.1, h);
            }
            Some(set) => {
                set.insert(beacon_pos);
            }
        }
        // println!("Distance from beacon at {beacon_pos:?} to sensor {sensor_pos:?} is {dist} ");

        let mut add_with_dist = |y: i32, dist: i32| {
            let vals = (sensor_pos.0 - dist, sensor_pos.0 + dist);
            let other = Range {
                from: std::cmp::min(vals.0, vals.1),
                to: std::cmp::max(vals.0, vals.1),
            };
            if (!rows.contains_key(&y)) {
                rows.insert(y, Vec::new());
            }
            let ranges = &mut rows.get_mut(&y);
            match ranges {
                None => {}
                Some(ranges) => {
                    let mut added = false;
                    for mut range in ranges.iter_mut() {
                        // If overlap

                        // 1a....2a...1b....2b
                        if (other.from <= range.from
                            && other.to <= range.to
                            && other.to >= range.from)
                        {
                            range.from = other.from;
                            added = true;
                            break;
                        }

                        // 2a .... 1a ...2b...1b
                        if (other.from >= range.from
                            && other.from <= range.to
                            && other.to >= range.to)
                        {
                            range.to = other.to;
                            added = true;
                            break;
                        }
                        // 1a ... 2a...2b ...1a
                        if (other.from <= range.from && other.to >= range.to) {
                            range.from = other.from;
                            range.to = other.to;
                            added = true;
                            break;
                        }

                        // 2a ... 1a..1b ..2b
                        if (other.from >= range.from && other.to <= range.to) {
                            added = true;
                            break;
                        }
                    }

                    if (!added) {
                        ranges.push(other);
                    }
                }
            }
        };
        // add_with_dist(sensor_pos.1, dist);
        if (sensor_pos != (8, 7)) {
            // return;
        }
        let y = sensor_pos.1;
        for (i, y) in (y..=y + dist).enumerate() {
            add_with_dist(y, dist - (i as i32));
        }
        let sth = (y - dist..y);
        let c = sth.clone().count();
        for (i, y) in sth.enumerate() {
            add_with_dist(y, dist - (c - i) as i32);
        }
    });

    // let mut sortedRows = rows.iter().collect::<Vec<_>>();
    // sortedRows.sort();
    // sortedRows.iter().for_each(|row| println!("{:?}", row));

    // rows.push(Ranges::One(Range { from: 5, to: 10 }));
    // rows.push(Ranges::One(Range { from: 6, to: 9 }));
    // rows.push(Ranges::More(vec![
    //     Range { from: 1, to: 3 },
    //     Range { from: 5, to: 8 },
    // ]));

    // print_rows(&rows);
    // rows[0].merge(&Range { from: 8, to: 12 });
    // print_rows(&rows);
    // let minX = rows
    //     .iter()
    //     .map(|x| x.1.iter().map(|r| r.from).min().unwrap())
    //     .min()
    //     .unwrap_or(0);
    // let maxX = rows
    //     .iter()
    //     .map(|x| x.1.iter().map(|r| r.to).max().unwrap())
    //     .max()
    //     .unwrap_or(35);
    // sortedRows.iter().for_each(|r| {
    //     let mut s = String::new();
    //     s += &r.0.to_string();
    //     for x in minX - 1..maxX + 2 {
    //         if r.1.iter().any(|r| r.is_in(x)) {
    //             s += "#";
    //         } else {
    //             s += "."
    //         }
    //     }
    //     println!("{s}");
    // });

    let b = beacons.get(&10).unwrap().len();
    let rowToCheck = rows.get_mut(&10).unwrap();
    rowToCheck.sort();
    let mut newRow: Vec<Range> = Vec::new();
    for i in rowToCheck.windows(2) {
        let mut range = i[0];
        let other = i[1];
        let mut added = false;

        if (other.from <= range.from && other.to <= range.to && other.to >= range.from) {
            range.from = other.from;
            added = true;
        }

        // 2a .... 1a ...2b...1b
        if (other.from >= range.from && other.from <= range.to && other.to >= range.to) {
            range.to = other.to;
            added = true;
        }
        // 1a ... 2a...2b ...1a
        if (other.from <= range.from && other.to >= range.to) {
            range.from = other.from;
            range.to = other.to;
            added = true;
        }

        // 2a ... 1a..1b ..2b
        if (other.from >= range.from && other.to <= range.to) {
            added = true;
        }
        if (added) {
            newRow.push(range);
        } else {
            newRow.push(range);
            newRow.push(other);
        }
    }
    newRow.sort();
    let s: i32 = newRow.iter().map(|r| r.sum()).sum();
    println!("{s}, {b}");
}

fn print_rows(rows: &Vec<Ranges>) {
    for row in rows {
        match row {
            Ranges::More(ranges) => println!("{ranges:?}"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Range {
    from: i32,
    to: i32,
}

impl Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.from, self.to)
    }
}
impl Debug for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.from, self.to)
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.from.partial_cmp(&other.from)
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.from.cmp(&other.from)
    }
}

impl Range {
    fn is_in(&self, n: i32) -> bool {
        if n >= self.from && n <= self.to {
            return true;
        } else {
            return false;
        }
    }
    fn sum(&self) -> i32 {
        return self.to - self.from;
    }
}

#[derive(Clone)]
enum Ranges {
    More(Vec<Range>),
}

impl Ranges {
    fn merge(&mut self, range: &Range) {
        match self {
            Self::More(vec) => {
                // Merge new range to first available one
                let mut added = false;
                for r in vec.iter_mut() {
                    if range.from < r.from && range.to >= r.from {
                        r.from = range.from;
                    }
                    if range.to > r.to && range.from < r.to {
                        r.to = range.to;
                    }
                    if added {
                        break;
                    }
                }
                if !added {
                    vec.push(*range);
                }
                // Reshuffle to potentially merge
                let mut newVec: Vec<Range> = Vec::new();
                for window in vec.iter().collect::<Vec<_>>().windows(2) {
                    let mut ranges = window.to_vec();
                    ranges.sort();
                    if ranges[0].to >= ranges[1].from {
                        newVec.push(Range {
                            from: ranges[0].from,
                            to: ranges[1].to,
                        })
                    } else {
                        newVec.push(*window[0]);
                        newVec.push(*window[1]);
                    }
                }
                if (vec.len() >= 2) {
                    *vec = newVec.into_iter().collect::<Vec<_>>();
                }
            }
        }
    }
}
