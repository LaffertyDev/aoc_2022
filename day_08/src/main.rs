use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Problem 1: {}", count_visible_trees(parse_forest(&contents)));
    println!("Problem 2: {}", find_highest_value_tree(parse_forest(&contents)));
}

fn parse_forest(input: &str) -> Vec<Vec<u32>> {
    return input.split('\n').map(|f| f.chars().map(|t| t.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();
}

fn count_visible_trees(forest: Vec<Vec<u32>>) -> u32 {
    let mut invisible_trees = 0;

    for y in 1..forest.len() - 1 {
        for x in 1..forest[y].len() - 1 {
            let mut visible_top = true;
            let mut visible_down = true;
            let mut visible_left = true;
            let mut visible_right = true;

            let potential_house = forest[y][x];
            for left in (0..=(x-1)).rev() {
                if forest[y][left] >= potential_house {
                    visible_left = false;
                    break;
                }

            }
            for right in (x+1)..forest[y].len() {
                if forest[y][right] >= potential_house {
                    visible_right = false;
                    break;
                }
            }
            for top in (0..=(y-1)).rev() {
                if forest[top][x] >= potential_house {
                    visible_top = false;
                    break;
                }
            }
            for down in (y+1)..forest.len() {
                if forest[down][x] >= potential_house {
                    visible_down = false;
                    break;
                }
            }

            if !visible_down && !visible_top && !visible_left && !visible_right {
                invisible_trees += 1;
            }
        }
    }

    (forest.len() * forest[0].len() - invisible_trees).try_into().unwrap()
}


fn find_highest_value_tree(forest: Vec<Vec<u32>>) -> u32 {
    let mut highest_value = 0;

    for y in 1..forest.len() - 1 {
        for x in 1..forest[y].len() - 1 {
            let mut visible_top = 0;
            let mut visible_down = 0;
            let mut visible_left = 0;
            let mut visible_right = 0;

            let potential_house = forest[y][x];
            for left in (0..=(x-1)).rev() {
                visible_left += 1;
                if forest[y][left] >= potential_house {
                    break;
                }
            }
            for right in (x+1)..forest[y].len() {
                visible_right += 1;
                if forest[y][right] >= potential_house {
                    break;
                }
            }
            for top in (0..=(y-1)).rev() {
                visible_top += 1;
                if forest[top][x] >= potential_house {
                    break;
                }
            }
            for down in (y+1)..forest.len() {
                visible_down += 1;
                if forest[down][x] >= potential_house {
                    break;
                }
            }

            highest_value = std::cmp::max(highest_value, visible_down * visible_top * visible_left * visible_right);
        }
    }

    highest_value
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses() {
        let input = "\
123
456
789";
        let mut assertion = Vec::new();
        assertion.push(Vec::new());
        assertion[0].push(1);
        assertion[0].push(2);
        assertion[0].push(3);
        assertion.push(Vec::new());
        assertion[1].push(4);
        assertion[1].push(5);
        assertion[1].push(6);
        assertion.push(Vec::new());
        assertion[2].push(7);
        assertion[2].push(8);
        assertion[2].push(9);

        assert_eq!(assertion, parse_forest(input));
    }

    #[test]
    fn first() {
        let input = "\
30373
25512
65332
33549
35390";
        assert_eq!(21, count_visible_trees(parse_forest(&input)));
    }

    #[test]
    fn second() {
        let input = "\
30373
25512
65332
33549
35390";
        assert_eq!(8, find_highest_value_tree(parse_forest(&input)));
    }
}