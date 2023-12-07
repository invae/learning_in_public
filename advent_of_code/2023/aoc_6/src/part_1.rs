#[allow(unused_imports)]
use ezlogs::{
    info,
    warn,
    errr,
};

pub fn entry(input: &str) -> String {
    warn("part_1 todo");
    let mut lines = input.split("\n");
    let times_header = lines.next().unwrap();
    let distances_header = lines.next().unwrap();
    let time_resource = parse_header(times_header);
    let distances_to_beat = parse_header(distances_header);

    let mut ans_d6_p1   = 1;
    let total_rounds = distances_to_beat.len();
    dbg!(&total_rounds);
    for index in 0..total_rounds {
        let mut time_held   = 1;
        let mut ways_to_win = 0;      
        while time_held < time_resource[index] {
            let time_allowd = time_resource[index];
            let time_remaining = get_time_to_move(time_held, time_allowd);
            let dist_traveld = get_dist_travelled(time_held, time_remaining);
            let record = distances_to_beat[index];
            let record_is_beat = dist_traveld > record;
            if record_is_beat {
                ways_to_win += 1;
            }
            time_held += 1;
        }
        dbg!(&ways_to_win);
        ans_d6_p1 = ans_d6_p1 * ways_to_win;
    }
    dbg!(&ans_d6_p1);

    return String::from(input);
}
fn parse_header(header: &str) -> Vec<i64>{
    let x = header.split(':');
    let x = x.last().unwrap();
    let x = x.split_ascii_whitespace();
    let mut ans = Vec::new();
    for item in x {
        let buf = str_to_int(item);
        ans.push(buf);
    } 
    return ans;
}
pub fn get_dist_travelled(time_held: i64, time_remaining: i64) -> i64 {
    return time_held * time_remaining;
}
pub fn get_time_to_move(time_held: i64, time_allowd: i64) -> i64 {
    return time_allowd - time_held;
}
pub fn str_to_int(some_str: &str) -> i64 {
    let x = some_str.trim().parse::<i64>().unwrap();
    return x;
}