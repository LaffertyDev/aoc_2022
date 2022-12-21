use regex::Regex;
use std::env;
use std::fs;
use std::collections::HashSet;

#[derive(PartialEq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode
}

struct ActionResult {
    ore_robots_built: u32,
    clay_robots_built: u32,
    obsidian_robots_built: u32,
    geode_robots_built: u32,
    ore_cost: u32,
    clay_cost: u32,
    obsidian_cost: u32,
    geode_cost: u32,
}

impl ActionResult {
    fn empty_result() -> ActionResult {
        ActionResult {
            ore_robots_built: 0,
            clay_robots_built: 0,
            obsidian_robots_built: 0,
            geode_robots_built: 0,
            ore_cost: 0,
            clay_cost: 0,
            obsidian_cost: 0,
            geode_cost: 0,
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_robot_ore_cost: u32,
    clay_robot_ore_cost: u32,
    obsidian_robot_ore_cost: u32,
    obsidian_robot_clay_cost: u32,
    geode_robot_ore_cost: u32,
    geode_robot_obsidian_cost: u32
}

impl Blueprint {
    fn get_max_ore_robots_needed(&self) -> u32 {
        self.ore_robot_ore_cost.max(self.clay_robot_ore_cost).max(self.obsidian_robot_ore_cost).max(self.geode_robot_ore_cost)
    }

    fn get_max_clay_robots_needed(&self) -> u32 {
        // 
        self.obsidian_robot_clay_cost
    }

    fn get_max_obsidian_robots_needed(&self) -> u32 {
        self.geode_robot_obsidian_cost
    }

    fn get_max_geode_robots_needed(&self) -> u32 {
        u32::MAX
    }
}

// Can produce 1 robot every turn
// 24 total turns .... 1..=24
// 24 total steps... and can do one of 5 actions -- Nothing, Build_Clay, Build_Ore, Build_Obsidian, Build_Geode
// so each board state has 5 ^ 24 possible simulations states... way too fucking many. How do I bring that down to something manageable?
    // only getting possible new states instead of impossible states could probably bring it down by a few factors
    // still not enough
    // considering only 4 possible states does bring it down A LOT... 281 trillion vs 59 quadrillion
    // first TWO states are always idle... so that means only 22 possible states
    // 24th minute can only be spent producing, so only 21 possible states
    // 4^21 == 4.3 trillion... still too many
    // if I do not ever produce more N robots where N is the maximum cost of any robot in that resource...
    // then I can safely bring it down to 3 possible states... idle / produce clay / produce osbsidian
    // 3 ^ 21 == 10 million possible states... doable
// don't even try and simulate AI, just navigate possible simulations and pick the one with the most Geodes after tick == 24
#[derive(PartialEq)]
enum Action {
    Idle,
    BuildRobot(Resource),
}

// Simulation is at the START of the simulation turn
struct SimulationState<'a> {
    tick_at_start: u32,
    simulation_blueprint: &'a Blueprint,
    action_to_perform: Action,

    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,

    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,

    has_idled_after_building_ore: bool,
    has_idled_after_building_clay: bool,
    has_idled_after_building_obsidian: bool
}

#[derive(Eq)]
#[derive(Hash)]
#[derive(PartialEq)]
struct SimulationStateResult {
    tick_built_during: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,

    has_idled_after_building_ore: bool,
    has_idled_after_building_clay: bool,
    has_idled_after_building_obsidian: bool
}

// I have no idea what this operation is called properly
fn fact_sum(num: u32) -> u32 {
    (1..=num).sum()
}

impl SimulationStateResult {
    fn get_available_actions(&self, blueprint: &Blueprint, tick_limit: u32) -> Vec<Action> {
        if self.tick_built_during == tick_limit - 1 {
            // There is nothing to do for the last generation, so just skip it
            return vec![Action::Idle];
        }

        if self.ore >= blueprint.geode_robot_ore_cost && self.obsidian >= blueprint.geode_robot_obsidian_cost && self.geode_robots < blueprint.get_max_geode_robots_needed() {
            // always build a geode if possible, ignoring everything else. This is always the most optimal path.
            return vec![Action::BuildRobot(Resource::Geode)];
        }

        // ignore anytime we're more than 2 over the cost of an item
        // that means we have already skipped this item
        let mut actions = vec![Action::Idle];
        if !self.has_idled_after_building_obsidian && self.ore >= blueprint.obsidian_robot_ore_cost && self.clay >= blueprint.obsidian_robot_clay_cost && self.obsidian_robots < blueprint.get_max_obsidian_robots_needed() {
            // if we can build an obsidian robot, we don't need to build a clay robot
            actions.push(Action::BuildRobot(Resource::Obsidian));
        }

        if !self.has_idled_after_building_clay && self.ore >= blueprint.clay_robot_ore_cost && self.clay_robots < blueprint.get_max_clay_robots_needed() {
            // if we can build a clay robot, we don't need to build an ore robot unless we're also waiting to build a more awesome robot
            actions.push(Action::BuildRobot(Resource::Clay));
        }

        if !self.has_idled_after_building_ore && self.ore >= blueprint.ore_robot_ore_cost && self.ore_robots < blueprint.get_max_ore_robots_needed() {
            actions.push(Action::BuildRobot(Resource::Ore));
        }

        actions
    }

    fn find_maximum_geodes_producible(&self, tick_limit: u32) -> u32 {
        let ticks_remaining_in_sim = tick_limit - self.tick_built_during + 3;

        // let obsidian_producible = self.obsidian + fact_sum(self.obsidian_robots);
        // let ore_producible = self.ore + fact_sum(self.ore_robots);

        self.geodes + (self.geode_robots * ticks_remaining_in_sim) + fact_sum(ticks_remaining_in_sim)
    }
}

impl SimulationState<'_> {
    fn can_purchase(&self, resource: Resource) -> bool {
        match resource {
            Resource::Ore => {
                return self.ore >= self.simulation_blueprint.ore_robot_ore_cost;
            },
            Resource::Clay => {
                return self.ore >= self.simulation_blueprint.clay_robot_ore_cost;
            },
            Resource::Obsidian => {
                return self.ore >= self.simulation_blueprint.obsidian_robot_ore_cost && self.clay >= self.simulation_blueprint.obsidian_robot_clay_cost;
            },
            Resource::Geode => {
                return self.ore >= self.simulation_blueprint.geode_robot_ore_cost && self.obsidian > self.simulation_blueprint.geode_robot_obsidian_cost;
            },
        }
    }

    fn perform_action(&self) -> ActionResult {
        match self.action_to_perform {
            Action::Idle => return ActionResult::empty_result(),
            Action::BuildRobot(Resource::Ore) => {
                return ActionResult {
                    ore_robots_built: 1,
                    clay_robots_built: 0,
                    obsidian_robots_built: 0,
                    geode_robots_built: 0,
                    ore_cost: self.simulation_blueprint.ore_robot_ore_cost,
                    clay_cost: 0,
                    obsidian_cost: 0,
                    geode_cost: 0,
                };
            },
            Action::BuildRobot(Resource::Clay) => {
                return ActionResult {
                    ore_robots_built: 0,
                    clay_robots_built: 1,
                    obsidian_robots_built: 0,
                    geode_robots_built: 0,
                    ore_cost: self.simulation_blueprint.clay_robot_ore_cost,
                    clay_cost: 0,
                    obsidian_cost: 0,
                    geode_cost: 0,
                };
            },
            Action::BuildRobot(Resource::Obsidian) => {
                return ActionResult {
                    ore_robots_built: 0,
                    clay_robots_built: 0,
                    obsidian_robots_built: 1,
                    geode_robots_built: 0,
                    ore_cost: self.simulation_blueprint.obsidian_robot_ore_cost,
                    clay_cost: self.simulation_blueprint.obsidian_robot_clay_cost,
                    obsidian_cost: 0,
                    geode_cost: 0,
                };            },
            Action::BuildRobot(Resource::Geode) => {
                return ActionResult {
                    ore_robots_built: 0,
                    clay_robots_built: 0,
                    obsidian_robots_built: 0,
                    geode_robots_built: 1,
                    ore_cost: self.simulation_blueprint.geode_robot_ore_cost,
                    clay_cost: 0,
                    obsidian_cost: self.simulation_blueprint.geode_robot_obsidian_cost,
                    geode_cost: 0,
                };
            },
        }
    }

    fn process_simulation(&self) -> SimulationStateResult {
        let optionally_built_robots = self.perform_action();

        SimulationStateResult {
            tick_built_during: self.tick_at_start,
            ore_robots: self.ore_robots + optionally_built_robots.ore_robots_built,
            clay_robots: self.clay_robots + optionally_built_robots.clay_robots_built,
            obsidian_robots: self.obsidian_robots + optionally_built_robots.obsidian_robots_built,
            geode_robots: self.geode_robots + optionally_built_robots.geode_robots_built,
            ore: self.ore + self.ore_robots - optionally_built_robots.ore_cost,
            clay: self.clay + self.clay_robots - optionally_built_robots.clay_cost,
            obsidian: self.obsidian + self.obsidian_robots - optionally_built_robots.obsidian_cost,
            geodes: self.geodes + self.geode_robots - optionally_built_robots.geode_cost,

            // if we have idled, but we could have bought this ore, then do nothing
            has_idled_after_building_ore: (self.has_idled_after_building_ore && self.action_to_perform != Action::BuildRobot(Resource::Clay)) || (self.action_to_perform == Action::Idle && self.can_purchase(Resource::Ore)),
            has_idled_after_building_clay: (self.has_idled_after_building_clay && self.action_to_perform != Action::BuildRobot(Resource::Obsidian)) || (self.action_to_perform == Action::Idle && self.can_purchase(Resource::Clay)),
            has_idled_after_building_obsidian: (self.has_idled_after_building_obsidian && self.action_to_perform != Action::BuildRobot(Resource::Geode)) || (self.action_to_perform == Action::Idle && self.can_purchase(Resource::Obsidian))
        }
    }

    fn new(blueprint: &Blueprint) -> SimulationState {
        SimulationState {
            tick_at_start: 1,
            simulation_blueprint: blueprint,
            action_to_perform: Action::Idle,

            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,

            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,

            has_idled_after_building_ore: false,
            has_idled_after_building_clay: false,
            has_idled_after_building_obsidian: false
        }
    }

    fn new_from_result<'a>(simulation_result: &SimulationStateResult, blueprint: &'a Blueprint, action: Action) -> SimulationState<'a> {
        SimulationState {
            tick_at_start: simulation_result.tick_built_during + 1,
            simulation_blueprint: blueprint,
            action_to_perform: action,

            ore_robots: simulation_result.ore_robots,
            clay_robots: simulation_result.clay_robots,
            obsidian_robots: simulation_result.obsidian_robots,
            geode_robots: simulation_result.geode_robots,

            ore: simulation_result.ore,
            clay: simulation_result.clay,
            obsidian: simulation_result.obsidian,
            geodes: simulation_result.geodes,

            has_idled_after_building_ore: simulation_result.has_idled_after_building_ore,
            has_idled_after_building_clay: simulation_result.has_idled_after_building_clay,
            has_idled_after_building_obsidian: simulation_result.has_idled_after_building_obsidian
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Problem 1: {}", problem_1(&contents));
    println!("Problem 2: {}", problem_2(&contents));
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    let re = Regex::new(r"(\d+)").unwrap();
    let lines = input.split('\n').filter(|l| l.len() > 0);
    let mut blueprints = vec![];
    for line in lines {
        let mut captures = re.captures_iter(line);
        blueprints.push(Blueprint {
            id: captures.next().unwrap()[1].parse::<u32>().unwrap(),
            ore_robot_ore_cost: captures.next().unwrap()[1].parse::<u32>().unwrap(),
            clay_robot_ore_cost: captures.next().unwrap()[1].parse::<u32>().unwrap(),
            obsidian_robot_ore_cost: captures.next().unwrap()[1].parse::<u32>().unwrap(),
            obsidian_robot_clay_cost: captures.next().unwrap()[1].parse::<u32>().unwrap(),
            geode_robot_ore_cost: captures.next().unwrap()[1].parse::<u32>().unwrap(),
            geode_robot_obsidian_cost: captures.next().unwrap()[1].parse::<u32>().unwrap()
        });
    }

    blueprints
}

fn process_blueprint(blueprint: &Blueprint, ticks_to_process: u32) -> u32 {
    let mut simulations_to_process = vec![];
    simulations_to_process.push(SimulationState::new(blueprint));
    let mut results_cache = HashSet::new();

    let mut max_result = 0;
    while let Some(simulation) = simulations_to_process.pop() {
        let simulation_result = simulation.process_simulation();
        if simulation_result.tick_built_during == ticks_to_process {
            max_result = max_result.max(simulation_result.geodes);
        } else {
            if !results_cache.contains(&simulation_result) {
                // determine if this path can result in a higher result than what we have
                if simulation_result.find_maximum_geodes_producible(ticks_to_process) > max_result {
                    let actions_from_result = simulation_result.get_available_actions(&blueprint, ticks_to_process);
                    for a in actions_from_result {
                        simulations_to_process.push(SimulationState::new_from_result(&simulation_result, &blueprint, a));
                    }
                }
            }
        }

        results_cache.insert(simulation_result);
    }

    max_result
}

fn problem_1(input: &str) -> u32 {
    let blueprints = parse_input(&input);

    let geodes: u32 = blueprints.iter().map(|b| process_blueprint(b, 24) * b.id).sum();

    return geodes;
}

fn problem_2(input: &str) -> u32 {
    let blueprints = parse_input(&input);

    let geodes = blueprints.iter().take(3).map(|b| process_blueprint(b, 32)).collect::<Vec<u32>>();

    return geodes.iter().product();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_parses() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
        let blueprints = parse_input(&input);
        let blueprint = blueprints.first().unwrap();

        assert_eq!(1, blueprint.id);
        assert_eq!(4, blueprint.ore_robot_ore_cost);
        assert_eq!(2, blueprint.clay_robot_ore_cost);
        assert_eq!(3, blueprint.obsidian_robot_ore_cost);
        assert_eq!(14, blueprint.obsidian_robot_clay_cost);
        assert_eq!(2, blueprint.geode_robot_ore_cost);
        assert_eq!(7, blueprint.geode_robot_obsidian_cost);
    }
    #[test]
    fn first() {
        let input = "\
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
        assert_eq!(33, problem_1(&input));
    }

    #[test]
    fn second() {
        let input = "\
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
        assert_eq!(56 * 62, problem_2(&input));
    }
}