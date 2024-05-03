use std::collections::VecDeque;

use itertools::Itertools;

fn main() {
    println!("Hello, world!");
    let mut state = State {
        robots: [1, 0, 0, 0],
        materials: [0, 0, 0, 0],
        minute: 0,
    };

    let mut leafs: Vec<State> = Vec::new();

    let mut queue: Vec<State> = vec![state];


    while let Some(s) = queue.pop() {
        // let s = queue[i];
        if s.minute <= 24 {
            queue.append(&mut s.expand())
        }
        // Remove duplicates
        queue = queue.into_iter().unique().collect();        
        
    }

    let res = leafs.iter().map(|s|s.materials[3]).max().unwrap();
    print!("{res}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    robots: [i32; 4],
    materials: [i32; 4],
    minute: i32,
}

const costs: [[i32; 4]; 4] = [[4, 0, 0, 0], [2, 0, 0, 0], [3, 14, 0, 0], [2, 0, 7, 0]];
const maxCosts: [i32;3] = [4, 14,7];

impl State {
    fn expand(&self) -> Vec<State> {
        let mut s = self.clone();
        s.minute += 1;
        // println!("Minute {}", s.minute);
        for (i, r) in s.robots.iter().enumerate() {
            s.materials[i] += r;
        }
        
        if (s.materials.into_iter().take(3).any(|s| s > 20)) {
            return vec![];
        }

        let mut res: Vec<State> = Vec::new();
        for (robot_i, robot_costs) in costs.into_iter().enumerate() {
            if robot_i < 3 && s.robots[robot_i] > maxCosts[robot_i] {
                continue;
            }
            if robot_costs
                .into_iter()
                .enumerate()
                .all(|(i, c)| self.materials[i] >= c)
            {
                let mut new_state = s.clone();

                // Remove all the materials
                new_state.materials = new_state
                    .materials
                    .into_iter()
                    .zip(robot_costs)
                    .map(|(a, b)| a - b)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                new_state.robots[robot_i] += 1;
                res.push(new_state);
            }
        }

        res.push(s);

        let max_geode_count = res.iter().map(|s| s.robots[3]).max().unwrap();

        let res = res
            .into_iter()
            .filter(|s| max_geode_count - s.robots[3] <= 0)
            // .filter(|s| s.robots.iter().zip(maxCosts).any(|(|))
            .collect::<Vec<_>>();

        // println!("Original state {self:?}");
        // for s in res.iter() {
        //     println!("Children {s:?}");
        // }
        return res;
    }
}
