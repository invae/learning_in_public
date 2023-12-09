#[allow(unused_imports)]
use ezlogs::{
    info,
    warn,
    errr,
};

pub fn entry(input: &str) -> String {
    warn("part_1 todo");
    let mut split_1 = input.split("\n\n");
    let lr_decider = split_1.next().unwrap().as_bytes();
    let unparsed_maps = split_1.last().unwrap().split('\n');

    let mut location_resolvers = Vec::new();
    for upm in unparsed_maps {
        let mut x = upm.split(" = ");
        let location = x.next().unwrap();
        let unparsed_resolver = x.last().unwrap();
        let x = unparsed_resolver.replace("(", "");
        let x = x.replace(")", "");
        let x = x.replace(",", "");
        let mut x = x.split_ascii_whitespace();
        let left = x.next().unwrap();
        let right = x.last().unwrap();

        let m = Map {
            left: left.to_string(),
            right: right.to_string(),
        };
        let l = LocationResolver {
            location: location.to_string(),
            resolver: m,
        };
        location_resolvers.push(l);
    }

    let modulo = Vec::from(lr_decider).iter().count();
    let mut final_ans: usize = 0;
    let mut current_index = get_index_of("AAA", &location_resolvers); // just read the file
    loop {
        if location_resolvers[current_index].location == "ZZZ" {
            break;
        }
        if lr_decider[final_ans%modulo] == b'L' {
            current_index = get_index_of(&location_resolvers[current_index].resolver.left, &location_resolvers)
        }
        if lr_decider[final_ans%modulo] == b'R' {
            current_index = get_index_of(&location_resolvers[current_index].resolver.right, &location_resolvers)
        }

        final_ans += 1;
    }
    dbg!(final_ans);

    return String::from(input);
}
fn get_index_of(some_str:&str, location_resolvers: &Vec<LocationResolver>) -> usize{
    let ans = 0;
    let range = location_resolvers.iter().count();
    for index in 0..range {
        if location_resolvers[index].location == some_str {
            return index as usize;
        }
    }
    errr("WE NEVER FOUND AN INDEX");

    return 69;
}
#[derive(Debug)]
#[derive(Clone)]
struct LocationResolver {
    location: String,
    resolver: Map,
}
#[derive(Debug)]
#[derive(Clone)]
struct Map {
    left: String,
    right: String,
}