#[allow(unused_imports)]
use ezlogs::{
    info,
    warn,
    errr,
};
#[allow(unused_imports)]
use crate::part_1;

#[allow(dead_code)]
pub fn entry(input: &str) -> String {
    warn("part_2 todo");
    let mut lines = input.split("\n");
    let times_header = lines.next().unwrap();
    let distances_header = lines.next().unwrap();
    let time_resource = parse_header(times_header);
    let distance_to_beat = parse_header(distances_header);

    let mut upper_bound = get_upper_bound(time_resource, distance_to_beat, time_resource / 2 );
    dbg!(&upper_bound);
    // dbg!(verify(52244393, 59707878, 430121812131276));
    while !verify(upper_bound, time_resource, distance_to_beat) {
        upper_bound -= 1;
    }
    dbg!(&upper_bound);
    dbg!(verify(upper_bound, time_resource, distance_to_beat));
    let mut lower_bound = get_lower_bound(time_resource, distance_to_beat, 2);
    dbg!(&lower_bound);
    while verify(lower_bound, time_resource, distance_to_beat) {
        lower_bound -= 1;
    }
    lower_bound += 1;
    dbg!(&lower_bound);
    dbg!(verify(lower_bound, time_resource, distance_to_beat));
    let ways_to_win = upper_bound - lower_bound + 1;    // +1 b.c counting not distance
    dbg!(ways_to_win);


    return String::from(input);
}
fn get_lower_bound(time_resource: u128, distance_to_beat: u128, time_held: u128) -> u128{
    // dbg!(&time_held);
    let mut ans = time_held;
    if verify(time_held, time_resource, distance_to_beat) {
        return  ans;
    }
    else {
        let x = time_held * 2;
        ans = get_lower_bound(time_resource, distance_to_beat, x);
        return ans;
    }
}
fn get_upper_bound(time_resource: u128, distance_to_beat: u128, time_held: u128) -> u128{
    // dbg!(&time_held);
    let mut ans = time_held;
    if !verify(time_held, time_resource, distance_to_beat) {
        return  ans;
    }
    else {
        let x = (time_resource - time_held) / 2;
        ans = get_upper_bound(time_resource, distance_to_beat, time_held + x);
        return ans;
    }
}
fn verify(time_held: u128, time_resource: u128, distance_to_beat: u128) -> bool {
    let run_time = time_resource - time_held;
    let speed = time_held;
    return distance_to_beat < (run_time * speed);
}
fn parse_header(header: &str) -> u128 {
    let x = header.split(':');
    let x = x.last().unwrap();
    let x:Vec<&str> = x.split_ascii_whitespace().collect();
    let x = x.join("");
    let x = part_1::str_to_int(&x) as u128;
    return x;
}