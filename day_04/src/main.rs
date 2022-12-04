use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let first_problem_contents = contents.clone();

    let assignments_do_overlap = first_problem_contents.split('\n')
        .map(|assignment| parse_assignment_ranges(assignment))
        .map(|(a1s, a1e, a2s, a2e)| does_assignment_pair_overlap(a1s, a1e, a2s, a2e))
        .filter(|ado| *ado == true)
        .count();

    let assignments_overlap_at_all = contents.split('\n')
        .map(|assignment| parse_assignment_ranges(assignment))
        .map(|(a1s, a1e, a2s, a2e)| does_assignment_pair_overlap_simple(a1s, a1e, a2s, a2e))
        .filter(|ado| *ado == true)
        .count();

    println!("Problem 1: {}", assignments_do_overlap);
    println!("Problem 2: {}", assignments_overlap_at_all);
}

fn parse_assignment_ranges(assignment: &str) -> (u32, u32, u32, u32) {
    let mut split_assignments = assignment.split(',');
    let mut a1 = split_assignments.next().unwrap().split('-');
    let mut a2 = split_assignments.next().unwrap().split('-');
    return (a1.next().unwrap().parse::<u32>().unwrap(), a1.next().unwrap().parse::<u32>().unwrap(), a2.next().unwrap().parse::<u32>().unwrap(), a2.next().unwrap().parse::<u32>().unwrap());
}

fn does_assignment_pair_overlap_simple(assignment_1_start: u32, assignment_1_end: u32, assignment_2_start: u32, assignment_2_end: u32) -> bool {
    (assignment_1_start >= assignment_2_start && assignment_1_start <= assignment_2_end) ||
    (assignment_2_start >= assignment_1_start && assignment_2_start <= assignment_1_end)
}

fn does_assignment_pair_overlap(assignment_1_start: u32, assignment_1_end: u32, assignment_2_start: u32, assignment_2_end: u32) -> bool {
    (assignment_1_start <= assignment_2_start && assignment_1_end >= assignment_2_end)
    || (assignment_2_start <= assignment_1_start && assignment_2_end >= assignment_1_end)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses() {
        assert_eq!((2,4,6,8), parse_assignment_ranges("2-4,6-8"));
    }

    #[test]
    fn find_overlaps() {
        assert!(does_assignment_pair_overlap(1,2,1,2));
        assert!(does_assignment_pair_overlap(1,3,1,2));
        assert!(does_assignment_pair_overlap(1,2,1,3));
        assert!(!does_assignment_pair_overlap(1,2,2,3));
        assert!(!does_assignment_pair_overlap(2,4,1,3));
    }

    #[test]
    fn find_overlap_at_all() {
        assert!(!does_assignment_pair_overlap_simple(2,4,6,8));
        assert!(does_assignment_pair_overlap_simple(5,7,7,9));
        assert!(does_assignment_pair_overlap_simple(2,8,3,7));
        assert!(does_assignment_pair_overlap_simple(6,6,4,6));
    }
}