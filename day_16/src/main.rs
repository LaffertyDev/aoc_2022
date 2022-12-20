use regex::Regex;
use std::env;
use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct Agent {
    current_position: String,
    current_destination: Option<String>,
    time_to_destination: Option<u32>
}

impl Agent {
    fn new(position: String) -> Agent {
        Agent {
            current_position: position,
            current_destination: None,
            time_to_destination: None
        }
    }
}

#[derive(Debug)]
struct Valve {
    id: String,
    flow_rate: u32,
    is_open: bool,
    destinations: Vec<String>
}

impl Valve {
    fn new(id: String, flow_rate: u32, destinations: Vec<String>) -> Valve {
        return Valve {
            id: id,
            flow_rate: flow_rate,
            is_open: false,
            destinations: destinations
        };
    }
}

struct ValveNetwork {
    valves: Vec<Valve>
}

impl ValveNetwork {
    fn reconstruct_path(&self, start: &str, end: &str, quickest_map: &HashMap<&str, &str>) -> Vec<String> {
        let mut current_pos = end;
        let mut path: Vec<String> = vec![current_pos.to_string()];
        while current_pos != start {
            current_pos = quickest_map.get(current_pos).unwrap();
            path.insert(0, current_pos.to_string());
        }

        path
    }

    fn get_valve_by_id(&self, position: &str) -> &Valve {
        return self.valves.iter().find(|v| v.id == position).unwrap();
    }

    fn find_shortest_path_from_valve_to_valve(&self, start_position: &str, target_position: &str) -> Option<Vec<String>> {
        // do not include current position
        let mut nodes_to_expand: Vec<(&Valve, u32)> = vec![];
        nodes_to_expand.push((self.get_valve_by_id(start_position), 0));
        let mut goal_cost = HashMap::new();
        goal_cost.insert(start_position.clone(), 0);
        let mut fastest_path = HashMap::new();

        // todo -- implement value function to transform from dijkstra to A*star
        while !nodes_to_expand.is_empty() {
            let evaluating_node = nodes_to_expand.pop().unwrap();
            if evaluating_node.0.id == target_position {
                return Some(self.reconstruct_path(start_position, target_position, &fastest_path));
            }

            let cost_to_get_to_current_node = *(goal_cost.get(&*evaluating_node.0.id).unwrap());
            for neighbor in &evaluating_node.0.destinations {
                let neighbor = self.get_valve_by_id(&neighbor);
                let cost_to_reach_neighbor = cost_to_get_to_current_node + 1; // cost is always increased by 1
                if !goal_cost.contains_key(&*neighbor.id) || cost_to_reach_neighbor < *goal_cost.get(&*neighbor.id).unwrap() {
                    goal_cost.entry(&*neighbor.id)
                        .and_modify(|cost| { *cost = cost_to_reach_neighbor })
                        .or_insert(cost_to_reach_neighbor);
                    fastest_path.entry(&neighbor.id).and_modify(|id| *id = &evaluating_node.0.id).or_insert(&evaluating_node.0.id);
                    if nodes_to_expand.iter().find(|n| n.0.id == neighbor.id).is_none() {
                        nodes_to_expand.push((neighbor, cost_to_reach_neighbor + 1));
                        nodes_to_expand.sort_by(|a,b| b.1.cmp(&a.1));
                    }
                }
            }
        }

        None
    }

    fn mark_valve_as_open(&mut self, position_to_close: &str) {
        self.valves.iter_mut().find(|v| v.id == position_to_close).unwrap().is_open = true;
    }

    fn produce_flow(&self) -> u32 {
        return self.valves.iter().filter(|v| v.is_open).map(|v| v.flow_rate).sum();
    }
}

fn compute_maxium_flow(valve_network: &ValveNetwork, start_position: &str, ticks_remaining: u32, simulated_closed: Vec<String>) -> u32 {
    if ticks_remaining == 0 {
        return 0;
    }

    println!("max flow: {}", ticks_remaining);

    let mut current_max_flow = 0;
    let potential_valves = valve_network.valves.iter().filter(|v| !v.is_open && v.flow_rate > 0 && !simulated_closed.contains(&v.id));
    for potential_valve in potential_valves {
        // visit valve and then compute all possible valves from this valve
        let path_to_valve = valve_network.find_shortest_path_from_valve_to_valve(start_position, &potential_valve.id).unwrap();

        let ticks_to_get_to_destination = path_to_valve.len() as u32 - 1; // do not count current position
        if (ticks_remaining - 1) < ticks_to_get_to_destination {
            continue; // this valve cannot be reached from this position, so it doesn't need computed
        }

        let ticks_remaining_after_travel_and_open = ticks_remaining - 1 - ticks_to_get_to_destination;
        // compute this valves value if it was opened now and left on
        let self_flow: u32 = ticks_remaining_after_travel_and_open * potential_valve.flow_rate;

        // now compute all possible child explorations
        let mut child_simulated_closed = simulated_closed.clone();
        child_simulated_closed.push(path_to_valve.last().unwrap().to_string());
        let child_maximum_flow = compute_maxium_flow(valve_network, path_to_valve.last().unwrap(), ticks_remaining_after_travel_and_open, child_simulated_closed);

        current_max_flow = current_max_flow.max(child_maximum_flow + self_flow);
    }

    current_max_flow
}

fn compute_maxium_flow_with_n_agents(valve_network: &ValveNetwork, agent_state: &mut Vec<Agent>, ticks_remaining: u32, simulation_opened: Vec<String>) -> u32 {
    if ticks_remaining == 0 {
        return 0;
    }

    let mut current_max_flow = 0;
    let potential_valves = valve_network.valves.iter().filter(|v| !v.is_open && v.flow_rate > 0 && !simulation_opened.contains(&v.id)).collect::<Vec<&Valve>>();

    let nonworking_agents = agent_state.iter().filter(|a| a.time_to_destination.is_none()).collect::<Vec<&Agent>>();
    if nonworking_agents.len() > 0 {
        // for every possible lever, compute maximum 
    }

    // we have agents that need to count down, find the amount to count down the clock
    let amount_to_tick_down = agent_state.iter().filter(|a| a.time_to_destination.is_some()).map(|a| a.time_to_destination.unwrap()).min().unwrap();
    agent_state.iter_mut().for_each(|a| a.time_to_destination = Some(a.time_to_destination.unwrap() - amount_to_tick_down));


    // for each agent
        // for each potential valve
            // if an agent is already going to this destination, don't repeat it
            // send to a destination, record its destination so other agents can't go there

    // each agent picks every possible value
    // keep the highest maximum flow currently recorded


    0

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Problem 1: {}", problem_1(&contents));
    println!("Problem 2: {}", problem_2(&contents));
}

fn parse_input(input: &str) -> ValveNetwork {
    let re = Regex::new(r"([A-Z]{2})|(\d+)").unwrap();
    let lines = input.split('\n').filter(|l| l.len() > 0);
    let mut valve_network = ValveNetwork {
        valves: vec![]
    };
    for line in lines {
        let mut title = None;
        let mut flow_rate = None;
        let mut destinations = vec![];
        for cap in re.captures_iter(line) {
            if title.is_none() {
                title = Some(cap[1].to_string());
            } else if flow_rate.is_none() {
                flow_rate = Some(cap[2].parse::<u32>().unwrap());
            } else {
                destinations.push(cap[1].to_string());
            }
        }

        let valve = Valve::new(title.unwrap(), flow_rate.unwrap(), destinations);
        valve_network.valves.push(valve);
    }

    valve_network
}

fn problem_1(input: &str) -> u32 {
    let valve_network = parse_input(&input);
    compute_maxium_flow(&valve_network, &"AA".to_string(), 30, vec![])
}

fn problem_2(input: &str) -> u32 {
    let valve_network = parse_input(&input);
    compute_maxium_flow_with_n_agents(&valve_network, &mut vec![Agent::new("AA".to_string()), Agent::new("AA".to_string())], 26, vec![])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first() {
        let input = "
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
        assert_eq!(1651, problem_1(&input));
    }

    #[test]
    fn second() {
        let input = "
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
        assert_eq!(1707, problem_2(&input));
    }
}