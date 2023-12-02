#[allow(unused_imports)]
use ezlogs::{
    info,
    warn,
    errr,
};
#[allow(unused_imports)]
use crate::part_1::{
    parse_line,
    Game,
    Round,
};

#[allow(dead_code)]
pub fn entry(input: &str) -> u32 {
    let lines = input.split('\n');

    let mut games = Vec::new();
    for l in lines {
        let buf = parse_line(l);
        games.push(buf);
    }

    let mut minimal_sets = Vec::new();
    for g in games {
        let buf = g.cube_golf();
        minimal_sets.push(buf);
    }

    let mut ans = 0; 
    for m in minimal_sets {
        ans += power(m);
    }
        
    return ans;
}
fn power(round: Round) -> u32 {
    let pwr = round.red * round.blue * round.green;
    return pwr;
}