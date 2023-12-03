use std::vec;

#[allow(unused_imports)]
use ezlogs::{
    info,
    warn,
    errr,
};

const MAGIC_LEN: usize = 140;

pub fn entry(input: &str) -> String {
    let x = string_math();
    let corner_case = String::from_utf8(x.to_vec()).unwrap();
    
    let lines: Vec<&str> = input.split('\n').collect();
    for line_num in 0..MAGIC_LEN {
        let curr_line = lines[line_num];
        // dbg!(String::from(curr_line));
        // make the pattern uniform, avoid first and last corner case
        match line_num {
            0 => {
                let prev_line = &corner_case;
                let next_line = lines[line_num+1];
                get_line_sum(prev_line, curr_line, next_line);
            },
            139 => {
                let prev_line = lines[line_num-1];
                let next_line = &corner_case;
                get_line_sum(prev_line, curr_line, next_line);
            }
            _ => {
                let prev_line = lines[line_num-1];
                let next_line = lines[line_num+1];
                get_line_sum(prev_line, curr_line, next_line);
            }
        }
    }


    return String::from(input);
}
fn get_line_sum(prev_line: &str, curr_line: &str, next_line: &str) {
    info("--- START LINE ANALSIS ---");
    errr(&String::from(prev_line));
    errr(&String::from(curr_line));
    errr(&String::from(next_line));
    // dbg!(prev_line);
    // dbg!(curr_line);
    // dbg!(next_line);
    let prev_line = Vec::from(prev_line);
    let curr_line = Vec::from(curr_line);
    let next_line = Vec::from(next_line);
    let mut sum_of_line = 0;
    for mut i in 0..MAGIC_LEN {
        if valid_symbol(curr_line[i]) &&  curr_line[i+1].is_ascii_digit() {    // we are lucky with this +1?
            // dbg!(String::from_utf8(curr_line.clone()).unwrap());
            // dbg!(x);
            // dbg!(condition_met(x));
            // info("SYMBOL DETECTED");
            // warn(&byte_to_ascii(x));
            // dbg!(&curr_line[i+1..]); 
            sum_of_line += yank_case_1(&curr_line[i+1..]);
            let l = get_len(&curr_line[i..]) as usize;
            i += l; // jump forward so as to not double count
            // break;
        }
        if valid_symbol(curr_line[i]) &&  curr_line[i-1].is_ascii_digit() {
            // dbg!(String::from_utf8(curr_line.clone()).unwrap());
            sum_of_line += yank_case_2(&curr_line[..i]);
        }
        if valid_symbol(prev_line[i]) || valid_symbol(next_line[i]){
            if curr_line[i].is_ascii_digit() {
                warn("corner case, handle it first");
                warn(&byte_to_ascii(prev_line[i]));
                warn(&byte_to_ascii(next_line[i]));
                // need to grab the whole word somehow
                continue;
            }
            if curr_line[i-1].is_ascii_digit() {
                warn("ADJACENCY AHEAD DETECTED");
                warn(&byte_to_ascii(prev_line[i]));
                warn(&byte_to_ascii(next_line[i]));
                // use yank2
                continue;
            }
            if curr_line[i+1].is_ascii_digit() {
                warn("ADJACENCY BEHIND DETECTED");
                warn(&byte_to_ascii(prev_line[i]));
                warn(&byte_to_ascii(next_line[i]));
                // use yank1
                continue;
            }
            // 12334
            // .....*
        }
    }
    // dbg!(sum_of_line);
    info("--- STOPP LINE ANALSIS ---");
}
fn yank_case_2(partition: &[u8]) -> u32 {
    info("YANK2");
    let mut x = partition.iter().rev();

    let mut buf = Vec::new();
    for item in x {
        if !item.is_ascii_digit() {
            break;
        }
        buf.push(*item);
    }
    buf.reverse();
    for item in buf.clone() {
        warn(&byte_to_ascii(item));
    }
    let sum = String::from_utf8(buf).unwrap();
    dbg!(sum);
    return 69;
}
fn yank_case_1(partition: &[u8]) -> u32 {
    info("YANK1");
    let mut part_number = Vec::new();
    for item in partition {
        if !item.is_ascii_digit() {
            break;
        }
        part_number.push(item.clone());
        warn("adding:");
        warn(&byte_to_ascii(*item));
    }
    let x = String::from_utf8(part_number).unwrap();
    // dbg!(x);
    let x = x.parse::<u32>().unwrap();
    
    return x;
}
pub fn valid_symbol(b: u8) -> bool {
    return !b.is_ascii_digit() && b != b'.';
}
pub fn get_len(partition: &[u8]) -> u32 {
    // dbg!(i);
    // dbg!(String::from_utf8(curr_line.to_vec()).unwrap());
    // let l = get_len(&curr_line[i..]);
    // dbg!(l);
    let mut  l = 0;
    for item in partition{
        if !item.is_ascii_digit() {
            // dbg!(String::from_utf8(partition.to_vec()).unwrap());
            break;
        }
        l += 1;
    }

    return l;
}
fn byte_to_ascii(b: u8) -> String {
    let x  = String::from_utf8(vec![b]).unwrap();
    return x;
}
fn string_math() -> [u8; MAGIC_LEN] {
    let mut a = [0; MAGIC_LEN];
    a[0] = b'.';
    for i in 0..MAGIC_LEN {
        a[i] = b'.';
    }

    return a;
}

/*  
we have at least 2 corner cases:
    1. first line, we have nothing above it to compare diagonals
    2. last line, we have nothing below it to compare diagonals
pseudo code:
    - get previous line
    - get current line
    - get next line
    - trace through these 3 lines, 
        if any offset is symbol, get that integer, append it to some collector
*/