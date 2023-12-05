#[allow(unused_imports)]
use ezlogs::{
    info,
    warn,
    errr,
};

pub fn entry(input: &str) -> usize {
    // prepare input data
    let lines = input.split('\n');
    let mut cards: Vec<Card> = Vec::new();
    for l in lines {
        let card_id = get_card_id(&l);
        let (win_nums, card_nums) = get_nums(&l);
        let buffer = Card {
            _id: card_id,
            winning_nums: win_nums,
            card_nums: card_nums,
        };
        cards.push(buffer);
    }
    // do action on data
    let mut sum = 0;
    for c in cards {
        sum += c.score();
    }

    info("part 1 complete!");
    return sum;
}



fn mega_parse(some_str: &str) -> Vec<u8> {
    let mut parsed: Vec<u8> = Vec::new();
    for item in some_str.split_ascii_whitespace() {
        parsed.push(item.parse().unwrap())
    }
    
    return parsed;
}
fn get_nums(l: &str) -> (Vec<u8>, Vec<u8>) {
    let mut x = l.split(':');
    x.next().unwrap();
    let x = x.next().unwrap();
    let mut x = x.split('|');
    let first = x.next().unwrap().trim();
    let last =  x.last().unwrap().trim();
    // let msg = format!("first nums: {}", first);
    // info(&msg);
    // let msg = format!("last nums: {}", last);
    // info(&msg);
    let first = mega_parse(first);
    let last  = mega_parse(last);


    return (first, last);
}
fn get_card_id(l: &str) -> u8 {
    let mut x = l.split(':');
    let x = x.next().unwrap();
    let mut x = x.split_ascii_whitespace();
    x.next().unwrap();
    let x = x.next().unwrap();
    // let msg = format!("card_id: {}", &x);
    // info(&msg);
    let x = x.parse().unwrap();

    return x;
}
/*
we can leverage exponential
maybe filters? make vec, filter vec where ?

*/
struct Card {
    _id: u8,
    winning_nums: Vec<u8>,
    card_nums:  Vec<u8>,
}
impl Card {
    pub fn score(&self) -> usize {
        let win_count = self.card_nums.iter().filter(|&i| self.winning_nums.contains(&i)).count();
        // dbg!(&win_count);
        let base: usize = 2;

        if win_count == 0 {
            return 0;
        } 
        let points = base.pow(win_count as u32 - 1);

        return points;
    }
}