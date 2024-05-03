use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::iter::Map;

use tuple::{TupleElements, T1, T3};

const EOL: &str = "\r\n";
const doubleEOL: &str = "\r\n\r\n";

fn main() {
    let mut file = File::open("./input.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");

    let cubes = contents
        .split(EOL)
        .map(|line| {
            line.split(",")
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let revealedSides = cubes
        .clone()
        .into_iter()
        .map(|c| get_sides_from_cube(c).to_vec())
        .flatten()
        .fold(HashMap::new(), |mut a: HashMap<Side, i32>, s| {
            match a.get_mut(&s) {
                None => {
                    a.insert(s, 1);
                }
                Some(n) => {
                    *n += 1;
                }
            }
            return a;
        });

    const dirs: [(i32, i32, i32); 6] = [
        (0, 0, 1),
        (0, 1, 0),
        (1, 0, 0),
        (0, 0, -1),
        (0, -1, 0),
        (-1, 0, 0),
    ];
    let cube_tup_set = cubes
        .clone()
        .into_iter()
        .map(|c| (c[0], c[1], c[2]))
        .collect::<HashSet<_>>();

    let minX = cube_tup_set.iter().map(|c| c.0).min().unwrap();
    let maxX = cube_tup_set.iter().map(|c| c.0).max().unwrap();
    let minY = cube_tup_set.iter().map(|c| c.1).min().unwrap();
    let maxY = cube_tup_set.iter().map(|c| c.1).max().unwrap();
    let minZ = cube_tup_set.iter().map(|c| c.2).min().unwrap();
    let maxZ = cube_tup_set.iter().map(|c| c.2).max().unwrap();
    let minmax = vec![(minX, maxX), (minY, maxY), (minZ, maxZ)];
    let mut list_of_cubes = Vec::new();
    for x in minX - 2..=maxX + 1 {
        for y in minY - 2..=maxY + 1 {
            for z in minZ - 2..=maxZ + 1 {
                list_of_cubes.push((x, y, z));
            }
        }
    }
    let coveredCubes = list_of_cubes
        .iter()
        .filter(|c| !cube_tup_set.contains(c))
        .filter(|side| {
            let side = side;
            dirs.iter().enumerate().all(|(i, (a, b, c))| {
                for i in 0..100 {
                    if cube_tup_set.contains(&(side.0 + a * i, side.1 + b * i, side.2 + c * i)) {
                        return true;
                    }
                }
                return false;
            })
        })
        .collect::<HashSet<_>>();

    let coveredSides = coveredCubes
        .clone()
        .into_iter()
        .map(|c| get_sides_from_cube(vec![c.0, c.1, c.2]).to_vec())
        .flatten()
        .fold(HashMap::new(), |mut a: HashMap<Side, i32>, s| {
            match a.get_mut(&s) {
                None => {
                    a.insert(s, 1);
                }
                Some(n) => {
                    *n += 1;
                }
            }
            return a;
        })
        .into_iter()
        .filter(|m| m.1 == 1)
        .collect::<HashSet<_>>();

    // for c in coveredCubes.iter() {
    //     println!("{c:?}");
    // }
    // let diff = z.difference(&r).collect::<Vec<_>>();
    let revealedSides = revealedSides
        .into_iter()
        .filter(|m| m.1 == 1)
        .collect::<HashSet<_>>();

    let rev_sides_len = revealedSides.len();
    let trulyRevealedSides = revealedSides
        .into_iter()
        .filter(|(side, _)| {
            let side = side.0;
            let p1 = side.0;
            let p2 = side.1;
            let (point1, point2) = (vec![p1.0, p1.1, p1.2], vec![p2.0, p2.1, p2.2]);

            let mut plane_i = 0;
            for i in 0..2 {
                if point1[i] == point2[i] {
                    plane_i = i;
                }
            }

            let (mut a, mut b) = (false, false);
            for i in (plane_i + 1) as i32..minmax[plane_i].1 + 2 {
                if cubes.iter().any(|c| do_vecs_match(c, &point1)) {
                    a = true;
                }
            }

            for i in minmax[plane_i].0 - 2..point1[plane_i] - 1 {
                if cubes.iter().any(|c| do_vecs_match(c, &point1)) {
                    b = true;
                }
            }
            return !(a && b);
        })
        .collect::<Vec<_>>();
    println!(
        "Found revealed sides {} on the outside are {} difference {}",
        rev_sides_len,
        trulyRevealedSides.len(),
        rev_sides_len - trulyRevealedSides.len()
    )
    // println!(
    //     "Intersection length {} normal length {}",
    //     res.len() - inter.count(),
    //     res.len()
    // );
}

fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

fn get_sides_from_cube(cube: Vec<i32>) -> [Side; 6] {
    let b = (cube[0], cube[1], cube[2]);

    return [
        Side((b, (b.0 + 1, b.1 + 1, b.2))),
        Side((b, (b.0, b.1 + 1, b.2 + 1))),
        Side((b, (b.0 + 1, b.1, b.2 + 1))),
        Side(((b.0 + 1, b.1, b.2), (b.0 + 1, b.1 + 1, b.2 + 1))),
        Side(((b.0, b.1 + 1, b.2), (b.0 + 1, b.1 + 1, b.2 + 1))),
        Side(((b.0, b.1, b.2 + 1), (b.0 + 1, b.1 + 1, b.2 + 1))),
    ];
}

type SideTuple = ((i32, i32, i32), (i32, i32, i32));
#[derive(Hash, Clone, Debug)]
struct Side(SideTuple);

impl Eq for Side {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialEq for Side {
    fn eq(&self, other: &Self) -> bool {
        if self.0 == other.0 {
            return true;
        }

        let side = self.0;

        let mut sideA = T3(side.0 .0, side.0 .1, side.0 .2);
        let mut sideB = T3(side.1 .0, side.1 .1, side.1 .2);
        let mut ii = 0;
        for i in 0..=2 {
            if sideA.get(i) == sideB.get(i) {
                ii = i;
            }
        }

        *sideA.get_mut(ii).unwrap() += 1;
        *sideB.get_mut(ii).unwrap() += 1;

        let sideSynonym: SideTuple = (sideA.into(), sideB.into());

        // println!(
        //     "{:?}, {:?}, {:?}",
        //     sideSynonym,
        //     other.0,
        //     sideSynonym == other.0
        // );
        if sideSynonym == other.0 {
            return true;
        }

        false
    }
}
