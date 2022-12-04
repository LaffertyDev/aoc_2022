use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("Problem 1: {}", compute_rps(&contents));
    println!("Problem 2: {}", compute_rps_winlosedraw(&contents));
}

fn compute_rps(contents: &str) -> u32 {
    let game_result = contents.split('\n').into_iter().map(|round| {
        if round == "" {
            return 0;
        }

        let mut round_map = round.chars();
        let opponent = match round_map.next().unwrap() {
            'A' => RockPaperScissorsHand::Rock,
            'B' => RockPaperScissorsHand::Paper,
            'C' => RockPaperScissorsHand::Scissors,
            _ => panic!()
        };

        round_map.next(); // ignore space
        let you = match round_map.next().unwrap() {
            'X' => RockPaperScissorsHand::Rock,
            'Y' => RockPaperScissorsHand::Paper,
            'Z' => RockPaperScissorsHand::Scissors,
            _ => panic!()
        };

        compute_game(opponent, you)
    }).sum::<u32>();

    game_result
}

fn compute_rps_winlosedraw(contents: &str) -> u32 {
    let game_result = contents.split('\n').into_iter().map(|round| {
        if round == "" {
            return 0;
        }

        let mut round_map = round.chars();
        let opponent = match round_map.next().unwrap() {
            'A' => RockPaperScissorsHand::Rock,
            'B' => RockPaperScissorsHand::Paper,
            'C' => RockPaperScissorsHand::Scissors,
            _ => panic!()
        };

        round_map.next(); // ignore space
        let you = match round_map.next().unwrap() {
            'X' => WinLoseDraw::Lose,
            'Y' => WinLoseDraw::Draw,
            'Z' => WinLoseDraw::Win,
            _ => panic!()
        };

        compute_game_win_lose_draw(opponent, you)
    }).sum::<u32>();

    game_result
}

enum WinLoseDraw {
    Win,
    Lose,
    Draw
}

enum RockPaperScissorsHand {
    Rock,
    Paper,
    Scissors
}

fn compute_game(opponent: RockPaperScissorsHand, you: RockPaperScissorsHand) -> u32 {
    let draw_point = 3;
    let win_point = 6;
    let lose_point = 0;

    let scissors_point = 3;
    let paper_point = 2;
    let rock_point = 1;
    match (opponent, you) {
        (RockPaperScissorsHand::Rock, RockPaperScissorsHand::Rock) => rock_point + draw_point,
        (RockPaperScissorsHand::Rock, RockPaperScissorsHand::Paper) => paper_point + win_point,
        (RockPaperScissorsHand::Rock, RockPaperScissorsHand::Scissors) => scissors_point + lose_point,
        (RockPaperScissorsHand::Paper, RockPaperScissorsHand::Rock) => rock_point + lose_point,
        (RockPaperScissorsHand::Paper, RockPaperScissorsHand::Paper) => paper_point + draw_point,
        (RockPaperScissorsHand::Paper, RockPaperScissorsHand::Scissors) => scissors_point + win_point,
        (RockPaperScissorsHand::Scissors, RockPaperScissorsHand::Rock) => rock_point + win_point,
        (RockPaperScissorsHand::Scissors, RockPaperScissorsHand::Paper) => paper_point + lose_point,
        (RockPaperScissorsHand::Scissors, RockPaperScissorsHand::Scissors) => scissors_point + draw_point,
    }
}

fn compute_game_win_lose_draw(opponent: RockPaperScissorsHand, you: WinLoseDraw) -> u32 {
    let draw_point = 3;
    let win_point = 6;
    let lose_point = 0;

    let scissors_point = 3;
    let paper_point = 2;
    let rock_point = 1;
    match (opponent, you) {
        (RockPaperScissorsHand::Rock, WinLoseDraw::Lose) => scissors_point + lose_point,
        (RockPaperScissorsHand::Rock, WinLoseDraw::Draw) => rock_point + draw_point,
        (RockPaperScissorsHand::Rock, WinLoseDraw::Win) => paper_point + win_point,
        (RockPaperScissorsHand::Paper, WinLoseDraw::Lose) => rock_point + lose_point,
        (RockPaperScissorsHand::Paper, WinLoseDraw::Draw) => paper_point + draw_point,
        (RockPaperScissorsHand::Paper, WinLoseDraw::Win) => scissors_point + win_point,
        (RockPaperScissorsHand::Scissors, WinLoseDraw::Lose) => paper_point + lose_point,
        (RockPaperScissorsHand::Scissors, WinLoseDraw::Draw) => scissors_point + draw_point,
        (RockPaperScissorsHand::Scissors, WinLoseDraw::Win) => rock_point + win_point,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compute_game_rock_rock_draw() {
        assert_eq!(4u32, compute_game(RockPaperScissorsHand::Rock, RockPaperScissorsHand::Rock))
    }

    #[test]
    fn compute_game_rock_paper_win() {
        assert_eq!(8u32, compute_game(RockPaperScissorsHand::Rock, RockPaperScissorsHand::Paper))
    }

    #[test]
    fn compute_game_rock_scissors_lose() {
        assert_eq!(3u32, compute_game(RockPaperScissorsHand::Rock, RockPaperScissorsHand::Scissors))
    }

    #[test]
    fn parse_game() {
        let data = "C X";

        assert_eq!(7u32, compute_rps(data))
    }

    #[test]
    fn parse_game_runs() {
        let data = "C X\nC X\n";

        assert_eq!(14u32, compute_rps(data))
    }

    #[test]
    fn parse_game_wld() {
        let data = "C X";

        assert_eq!(2u32, compute_rps_winlosedraw(data))
    }

    #[test]
    fn parse_game_runs_wld() {
        let data = "C X\nC X\n";

        assert_eq!(4u32, compute_rps_winlosedraw(data))
    }
}