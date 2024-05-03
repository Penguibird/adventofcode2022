use std::fs::File;
use std::io::prelude::*;

const EOL: &str = "\r\n";
const doubleEOL: &str = "\r\n\r\n";

#[derive(Clone)]
struct Point {
    visited: bool,
    distance: i32,
    previous_node: Option<[i32; 2]>,
}
// struct Point<'a> {
//     coords: [i32; 2],
//     adjacent: Vec<&'a Point<'a>>,
//     // adjacent: Vec<[i32;2]>,
// }
fn main() {
    let mut file = File::open("./realinput.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");

    let grid = contents
        .split(EOL)
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|c| c as u32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut starting_pos = [0, 0];
    let mut goal_pos = [0, 0];

    let mut distance_table = vec![
        vec![
            Point {
                visited: false,
                distance: i32::MAX,
                previous_node: None
            };
            grid[0].len()
        ];
        grid.len()
    ];
    // let mut dijkstra = vec![vec![vec![[0, 0]]; grid[0].len()]; grid.len()];
    println!("{:?}", grid);

    let mut starting_points: Vec<[i32; 2]> = Vec::new();

    for y in 0..grid.len() {
        let line = &grid[y];
        for x in 0..line.len() {
            let char = line[x];
            if char == ('S' as u32) {
                starting_pos = [x, y];
                distance_table[y][x].distance = 0;
            }
            if char == ('a' as u32) {
                starting_points.push([x as i32, y as i32]);
                // distance_table[y][x].distance = 0;
            }
            if char == ('E' as u32) {
                for (dx, dy) in DIRS {
                    let ny = y as i32 + dy;
                    let nx = x as i32 + dx;
                    if ny < 0 || ny >= distance_table.len() as i32 {
                        continue;
                    }

                    if nx < 0 || nx >= distance_table[ny as usize].len() as i32 {
                        continue;
                    }
                    let ny = ny as usize;
                    let nx = nx as usize;

                    if grid[ny][nx] == ('z' as u32) {
                        goal_pos = [nx, ny];
                    }
                }
            }
        }
    }

    // for pos in starting_points;
    let mut dists = starting_points
        .iter()
        .map(|pos| {
            let mut d = vec![
                vec![
                    Point {
                        visited: false,
                        distance: i32::MAX,
                        previous_node: None
                    };
                    grid[0].len()
                ];
                grid.len()
            ];
            d[pos[1] as usize][pos[0] as usize].distance = 0;

            println!("Starting at {:?}", pos);
            visit(&grid, pos[0] as usize, pos[1] as usize, &mut d);
            // d.iter()
            //     .map(|l| l.iter().map(|a| a.distance).collect::<Vec<_>>())
            //     .for_each(|l| {
            //         println!("{:?}", l);
            // });

            d[goal_pos[1]][goal_pos[0]].distance
        })
        .collect::<Vec<_>>();
    dists.sort();

    println!("Goal pos {:?}", goal_pos);

    println!(
        "Result is {}",
        // distance_table[goal_pos[1]][goal_pos[0]].distance // dists[0]
        dists[0] + 1
    );
    // println!("{:?}", adjacency_table);
}

const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];

fn visit(grid: &Vec<Vec<u32>>, x: usize, y: usize, distance_table: &mut Vec<Vec<Point>>) {
    let char = grid[y][x];
    let current_dist = distance_table[y][x].distance;

    // println!("Visiting point at {} {}", x, y);

    for (dy, dx) in DIRS.iter() {
        let ny = y as i32 + dy;
        let nx = x as i32 + dx;
        if ny < 0 || ny >= distance_table.len() as i32 {
            continue;
        }

        if nx < 0 || nx >= distance_table[ny as usize].len() as i32 {
            continue;
        }
        let ny = ny as usize;
        let nx = nx as usize;

        if (grid[ny][nx] as i32) - (char as i32) <= 1 || grid[ny][nx] == ('a' as u32) {
            // If we can move there
            let point = &mut distance_table[ny][nx];
            if current_dist + 1 < point.distance {
                point.visited = true;
                point.distance = current_dist + 1;
                point.previous_node = Some([x as i32, y as i32]);
                visit(grid, nx, ny, distance_table);
            }
        }

        // if (grid[y][x] as i32) - (char as i32) == 1 {
        //     if adjacency_table.get(y) == None {
        //         adjacency_table[y] = vec![vec![[0, 0]]];
        //     }
        //     adjacency_table[y][x].push([y, x]);
        // }
    }
}
