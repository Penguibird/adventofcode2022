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

    let parsed = contents
        .split(EOL)
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    point
                        .split(",")
                        .map(|c| {
                            // println!("{c}");
                            c.parse::<i32>().unwrap()
                        })
                        .collect::<Vec<_>>()
                })
                // .map(|vec| (vec[0], vec[1]))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let minX = parsed
        .iter()
        .flatten()
        .map(|p| p[0])
        .fold(i32::MAX, |acc, e| if e < acc { e } else { acc });
    let minY = parsed
        .iter()
        .flatten()
        .map(|p| p[1])
        .fold(i32::MAX, |acc, e| if e < acc { e } else { acc });

    let maxX = parsed
        .iter()
        .flatten()
        .map(|p| p[0])
        .fold(0, |acc, e| if e > acc { e } else { acc });
    let maxY = parsed
        .iter()
        .flatten()
        .map(|p| p[1])
        .fold(0, |acc, e| if e > acc { e } else { acc });

    let mut grid = vec![vec![Point::Empty; 1000]; 1000];

    for path in parsed {
        for points in path.windows(2) {
            // let dx = points[0][0] - points[1][0];
            // let dy = points[0][1] - points[1][1];
            // println!("Drawing line at {dx} {dy}");
            println!("{points:?}");
            let mut points = vec![points[0].clone(), points[1].clone()];
            points.sort();

            for x in (points[0][0]..=points[1][0]) {
                for y in (points[0][1]..=points[1][1]) {
                    println!("{x} {y}");
                    grid[y as usize][x as usize] = Point::Wall;
                }
            }
        }
    }

    // Build infinite floor, part 2
    for x in minX - maxY..=maxX + maxY {
        grid[(maxY + 2) as usize][x as usize] = Point::Wall;
    }

    // Walls are built, sand simulation

    let mut i = -1;
    'outer: loop {
        i += 1;
        let mut sand_pos = (500, 0);
        loop {
            if grid[sand_pos.1 + 1][sand_pos.0] == Point::Empty {
                sand_pos.1 += 1;
            } else if grid[sand_pos.1 + 1][sand_pos.0 - 1] == Point::Empty {
                sand_pos.1 += 1;
                sand_pos.0 -= 1;
            } else if grid[sand_pos.1 + 1][sand_pos.0 + 1] == Point::Empty {
                sand_pos.1 += 1;
                sand_pos.0 += 1;
            } else if sand_pos.0 == 500 && sand_pos.1 == 0 {
                println!("Result {}", i + 1);
                break 'outer;
            } else {
                grid[sand_pos.1][sand_pos.0] = Point::Sand;
                break;
            }
        }
    }

    display_wall(&grid, minX as usize, maxX as usize, 0, maxY as usize);
}

fn display_wall(grid: &Vec<Vec<Point>>, minX: usize, maxX: usize, minY: usize, maxY: usize) {
    for line in grid[minY..=maxY].iter() {
        let n = line[minX..=maxX]
            .iter()
            .map(|p| match p {
                Point::Empty => ".",
                Point::Wall => "#",
                Point::Sand => "o",
            })
            .collect::<Vec<_>>()
            .join("");
        println!("{n}");
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Point {
    Empty,
    Wall,
    Sand,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Point::Empty => write!(f, "."),
            Point::Wall => write!(f, "#"),
            Point::Sand => write!(f, "o"),
        }
    }
}
