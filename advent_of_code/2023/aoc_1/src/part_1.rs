#[allow(unused_imports)]
use ezlogs::{
    info,
    warn,
    errr,
};
use std::str;

pub fn entry(input: &str) -> i32 {
    info("part 1 begins");
    let digits = parse_lines(input);
    let sum = compute_sum(digits);

    return sum;
}
fn compute_sum(digits: Vec<Vec<u8>>) -> i32 {
    info("computing sum");
    let mut answer:i32 = 0;
    // info(&answer.to_string());
    for d in digits {
        let buffer = String::from_utf8(d).unwrap();
        // info(&buffer);
        answer += buffer.parse::<i32>().unwrap();
        
    }
    info(&answer.to_string());
    
    return answer;
}
fn parse_lines(input: &str) -> Vec<Vec<u8>> {
    info("collecting all digits");
    let mut output = Vec::new();
    for line in input.split('\n') {
        let mut buffer = Vec::new();
        match count(line) {
            1 => {
                // warn("only one digit");
                buffer.push(get_first(line));
                buffer.push(get_first(line));
            },
            _ => {
                buffer.push(get_first(line));
                buffer.push(get_last(line));
            }
        }
        output.push(buffer.clone());
        // dbg!(String::from_utf8_lossy(&buffer));
        // let x: &str = str::from_utf8(&buffer).unwrap();
        // info(x);
    }
    return output;
}
fn get_first(line: &str) -> u8 {
    let mut buffer = Vec::new();
    for c in line.bytes() {
        if c.is_ascii_digit() {
            buffer.push(c);
            break;
        }
    }
    return buffer[0];
}
fn get_last(line: &str) -> u8 {
    let mut buffer = Vec::new();
    for c in line.bytes().rev() {
        if c.is_ascii_digit() {
            buffer.push(c);
            break;
        }
    }
    return buffer[0];
}
fn count(line: &str) -> u8 {
    let mut count = 0;
    for c in line.bytes() {
        if c.is_ascii_digit() {
            count += 1;
        }
    }
    return count;
}

#[allow(dead_code)]
fn oops_didnt_read() {
    let line = String::new();
    let mut byte_iter = line.bytes();
    let first = byte_iter.nth(0).unwrap();
    let last  = byte_iter.last();
    
    let mut result = Vec::new();
    match last {
        Some(x) => {
            result.push(first);
            result.push(x);
        },
        None  => {
            warn("none type on last");
            result.push(first);
        },
    }
    dbg!(String::from_utf8(result).unwrap());
}