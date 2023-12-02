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
    println!("Answer to part 1: {}",output_1);
    let output_2 = part_2::entry(input.trim());
    println!("Answer to part 2: {}",output_2);
    // let msg = format!("input.txt contents: {}", output_2);
    // info(&msg);
    // errr("solution incomplete");
}
