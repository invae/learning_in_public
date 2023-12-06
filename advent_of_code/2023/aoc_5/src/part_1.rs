#[allow(unused_imports)]
use ezlogs::{
    info,
    warn,
    errr,
};
use core::panic;
use std::iter::zip;

pub fn entry(input: &str) -> String {
    // ingest data
    let lines = input.split("\n\n");
    let mut seeds: Vec<String> = Vec::new();
    let mut maps: Vec<Item2ItemMap> = Vec::new();
    for item in lines.enumerate() {
        if item.0 == 0 {
            parse_seed_header(item.1, &mut seeds);
            continue;
        }
        // let header = item.1.split(':').next().unwrap();
        // dbg!(header);
        let x = Item2ItemMap::new(item.1);
        maps.push(x);
    }
    
    let z = seeds.iter();
    let mut output: Vec<_Seed> = Vec::new();
    for item in seeds {
        let mut s = _Seed {
            meta_location: String::from("seed"),
            fertilizer: String::new(),
            humidity: String::new(),
            light: String::new(),
            location: String::new(),
            seed: String::from(item.to_string()),
            soil: String::new(),
            temperature: String::new(),
            water: String::new(),
        };
        while s.meta_location == "seed" {
            let x = maps.clone();
            for m in x {
                if m.src_item == s.meta_location{
                    let msg = format!("mapping : {}->{}", m.src_item, m.dst_item);
                    warn(&msg);
                    match s.meta_location.as_str() {
                        // this whole block could be avoided if we used a map
                        "seed" => {
                            s.soil = m.compute(&s.seed);
                        },
                        "soil" => {
                            s.fertilizer = m.compute(&s.soil);
                        },
                        "fertilizer" => {
                            s.water  = m.compute(&s.fertilizer);
                        },
                        "water" => {
                            s.light = m.compute(&s.water);
                        },
                        "light" => {
                            s.temperature = m.compute(&s.light);
                        },
                        "temperature" => {
                            s.humidity = m.compute(&s.temperature);
                        },
                        "humidity" => {
                            s.location = m.compute(&s.humidity);
                        },
                        _ => errr("who put this here?!")
                    }
                    s.meta_location = m.dst_item;
                }
            }
        }
        dbg!(&s);
        output.push(s);
    }
    // action on data
    // let x = output.iter().min();
    let mut x = Vec::new();
    for item in output { 
        // dbg!(&item.location);
        x.push(str_to_int(&item.location));
    }
    let ans = x.iter().min().unwrap();
    dbg!(ans);

    return String::from(input);
}
fn parse_seed_header(header: &str, storage: &mut Vec<String>) {
    let x = header.split(":");
    let x = x.last().unwrap().trim();
    let x = x.split(' ');
    for item in x {
        storage.push(String::from(item));
    }
}
#[derive(Debug)]
#[derive(Clone)]
struct _Seed {
    meta_location: String,
    fertilizer: String,
    humidity: String,
    light: String,
    location: String,
    seed: String,
    soil: String,
    temperature: String,
    water: String,
}
impl _Seed {
    pub fn _get_val(&self) -> u64 {
        return 69;
    }
}
#[derive(Debug)]
#[derive(Clone)]
struct Item2ItemMap {
    src_item: String,
    dst_item: String,
    maps: Vec<String>,
}
impl Item2ItemMap {
    pub fn new(input_block: &str) -> Self {
        let header = input_block.split(" map:").next().unwrap();
        let src = header.split("-to-").next().unwrap();
        let dst = header.split("-to-").last().unwrap();
        let map_str = input_block.split(" map:").last().unwrap();
        let mut unparse_maps: Vec<String> = Vec::new();
        for upm in map_str.trim().split('\n') {
            let buf = String::from(upm);
            unparse_maps.push(buf);
        };
        let new = Item2ItemMap {
            src_item: src.to_string(),
            dst_item: dst.to_string(),
            maps: unparse_maps,
        };

        return new;
    }
}
impl Item2ItemMap {
    pub fn compute(&self, input: &str) -> String {
        let input = str_to_int(input);
        // dbg!(input);
        let mut ans = input; // do no action if rules are not satisifed
        let maps_clone = self.maps.clone();
        for item in maps_clone {
            let mut tokens = item.split(" ");
            let dst_lwr = str_to_int(tokens.next().unwrap());
            let src_lwr = str_to_int(tokens.next().unwrap());
            let range = str_to_int(tokens.next().unwrap());
            let src_upper = src_lwr + range;
            // let dst_upper = dst_lwr + range;
            // rules
            // if input < src_lwr then continue    
            // if input > src_lwr+range then continue
            // otherwise compute the map
            if input < src_lwr {
                continue;
            }
            else if src_upper < input {
                continue;
            }
            // compute the map
            info("input is in range, computing map!");
            // dbg!(dst_lwr);
            // dbg!(src_lwr);
            // dbg!(range);
            // dbg!(input);

            let index = input - src_lwr;
            ans = dst_lwr + index;
            // dbg!(ans);
        }

        return ans.to_string();
    }
}

fn str_to_int(some_str: &str) -> u64 {
    let x = some_str.trim().parse::<u64>().unwrap();

    return x;
}
