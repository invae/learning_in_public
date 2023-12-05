mod part_1;
mod part_2;
mod tests;
#[allow(unused_imports)]
use ezlogs::{
    info,
    warn,
    errr,
};

fn main() {
    let input = include_str!("../input.txt");
    let output_1 = part_1::entry(input.trim());
    let msg = format!("part 1 ans: {}", output_1);
    info(&msg);
    // let output_2 = part_2::entry(&output_1);
    // let msg = format!("input.txt contents: {}", output_2);
    // info(&msg);
    errr("day incomplete");
}
