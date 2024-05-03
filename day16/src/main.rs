use std::collections::HashMap;

const input: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

const EOL: &str = "
";

#[derive(Clone, Debug, PartialEq)]
struct Valve<'a> {
    flowRate: i32,
    tunnels: Vec<&'a str>,
}
fn main() {
    let mut nodes: HashMap<String, Valve> = HashMap::new();
    input.split(EOL).for_each(|line| {
        let x = line.split(" has flow rate=").collect::<Vec<_>>();
        let valve = x[0].replace("Valve ", "");
        let x = x[1].split("; tunnels lead to valves ").collect::<Vec<_>>();
        let flowRate = x[0].parse::<i32>().unwrap();
        let tunnels = x[1].split(", ").collect::<Vec<_>>();
        nodes.insert(valve, Valve { flowRate, tunnels });
    })

    
}
