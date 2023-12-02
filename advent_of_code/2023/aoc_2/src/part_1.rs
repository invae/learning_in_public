#[allow(unused_imports)]
use ezlogs::{
    info,
    warn,
    errr,
};
struct Constraints {
    red_count: u32,
    green_count: u32,
    blue_count: u32,
}
const LIMITATIONS: Constraints =  Constraints {
    red_count: 12,
    green_count: 13,
    blue_count: 14,
};

pub fn entry(input: &str) -> u32 {
    let lines = input.split('\n');

    let mut games = Vec::new();
    for l in lines {
        let buf = parse_line(l);
        games.push(buf);
    }

    let feasible_games:Vec<Game> = games.into_iter().filter(|x| is_feasible(x)).collect();
    let sum = sum_game_ids(feasible_games);

    return sum;
}
fn sum_game_ids(games:Vec<Game>) -> u32 {
    let mut sum = 0;
    for g in games {
        sum += g.id;
    }
    return sum;
}
fn is_feasible(x:&Game) -> bool {
    let tmp = x.rounds.clone();
    for item in tmp {
        if item.red > LIMITATIONS.red_count {
            return false;
        }
        if item.green > LIMITATIONS.green_count {
            return false;
        }
        if item.blue > LIMITATIONS.blue_count {
            return false;
        }
    };

    return  true;
}
pub fn parse_line(line: &str) -> Game {
    let mut tmp = line.split(':');
    let id = tmp.next().unwrap().split(' ').last().unwrap();
    
    let round_data = tmp.next().unwrap().split(';');
    let mut rounds = Vec::new();
    for r in round_data {
        let parsed_round = parse_round(r);
        rounds.push(parsed_round);
    }
    let game = Game {
        id: id.parse::<u32>().unwrap(),
        rounds: rounds,
    };
    // dbg!(&game);
    return game;
}
fn parse_round(line: &str) -> Round {
    let mut round = Round::new();
    let tmp = line.trim();
    let colors = tmp.split(',');
    for c in colors {
        let c = c.trim();
        let mut c = c.split(' ');
        let num = c.next().unwrap();
        let color = c.last().unwrap();
        match color {
            "red" => round.red = num.parse::<u32>().unwrap(),
            "green" => round.green = num.parse::<u32>().unwrap(),
            "blue" => round.blue = num.parse::<u32>().unwrap(),
            _ => errr("unknown color"),
        };
    }

    return round;
}
#[derive(Debug)]
#[derive(Clone)]
pub struct Round {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}
impl Round {
    pub fn new() -> Round {
        let x = Round {
            red: 0,
            green: 0,
            blue: 0,
        };
        return x;
    }   
}
#[derive(Debug)]
#[derive(Clone)]
pub struct  Game {
    id: u32,
    rounds: Vec<Round>,
}
impl Game {
    pub fn cube_golf(&self) -> Round{
        // computes the min cubes reqqured to play a game
        let mut minimal_cube_count = Round::new();
        for r in &self.rounds {
            if r.red > minimal_cube_count.red {
                minimal_cube_count.red = r.red;
            }
            if r.green > minimal_cube_count.green {
                minimal_cube_count.green = r.green;
            }
            if r.blue > minimal_cube_count.blue {
                minimal_cube_count.blue = r.blue;
            }
        }
        return minimal_cube_count;
    }
}