use core::num;
use std::collections::HashSet;
use std::iter::FromIterator;
#[allow(unused_imports)]
use ezlogs::{
    info,
    warn,
    errr,
};

pub fn entry(input: &str) -> String {
    let lines  = input.split('\n');
    let mut hands = Vec::new();
    let mut bids = Vec::new();
    for l in lines {
        let mut x = l.split_ascii_whitespace();
        hands.push(x.next().unwrap());
        bids.push(str_to_int(x.next().unwrap()));
    }
    
    let count = hands.iter().count();
    let mut buf = Vec::new();
    for i in 0..count {
        buf.push((hands[i],bids[i]));
    }
    // get_num_occur(hands[24]);
    // buf.sort_by(compare)
    // get max num of occurences in both lines
    // if max_0 < max_1 then swap, set some flag to true
    let length = buf.clone().iter().count();
    for index in 0..length {
        if index == length-1 {
            break;
        }
        let x = get_num_occur(buf[index].0);
        let y = get_num_occur(buf[index+1].0);
        if x.len() == 2 && y.len() == 2 {
            // we are on the edge case with pairs
            // make advanced comparison, set flag accordingly
            continue;
        }
        match x.len() {
            2 => {
                warn("potential edge case");
                if y.len() == 2 {
                    warn("confirmed edge case 2==2");
                }
            },
            _ => {
                info("normal ordering");
                if x[0].1 < y[0].1 {
                    // do hanoi switch, set flag true
                    continue;
                }
                // otherwise do nothing
            },
        }
        // dbg!(x);
    }

    return String::from(input);
}
fn _hashset(data: &[u8]) -> HashSet<u8> {
    HashSet::from_iter(data.iter().cloned())
}
pub fn str_to_int(some_str: &str) -> u32 {
    let x = some_str.trim().parse::<u32>().unwrap();
    return x;
}

fn get_num_occur(hand: &str) -> Vec<(u8, usize)> {
//count num occurrences:
    let x:Vec<u8> = hand.bytes().collect();
    let values = _hashset(&x);
    let mut winners = Vec::new();
    winners.push((0,0));
    // (value, num_occur)
    // winners.pop();  
    dbg!(&values.len());
    for item in values {
        let y = x.clone();
        let num_occur = y.iter().filter(|&b| *b == item).count();  // count num occurences of any val
        // dbg!(&item);
        // dbg!(num_occur);
        winners.sort();
        let king_of_the_hill = winners.iter().last().unwrap().1;
        errr("'-----'");
        if num_occur > king_of_the_hill {
            // dbg!(&winners);
            winners.clear();    
            // let mut winners = Vec::new();
            winners.push((item, num_occur));
        }
        if num_occur == king_of_the_hill {
            dbg!(&winners);
            winners.push((item, num_occur));
            dbg!(&winners);
        }
    }
    // dbg!(&winners.len());
    return winners;
}