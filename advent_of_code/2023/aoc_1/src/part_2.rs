#[allow(unused_imports)]
use ezlogs::{
    info,
    warn,
    errr,
};
#[allow(unused_imports)]
use crate::part_1;
use std::slice::Iter;
use std::str;

#[derive(Clone)]
enum WordDigit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}
impl WordDigit {
    fn string(&self) -> String {
        match *self {
            WordDigit::Zero    => "zero".to_string(),
            WordDigit::One     => "one".to_string(),
            WordDigit::Two     => "two".to_string(),
            WordDigit::Three   => "three".to_string(),
            WordDigit::Four    => "four".to_string(),
            WordDigit::Five    => "five".to_string(),
            WordDigit::Six     => "six".to_string(),
            WordDigit::Seven   => "seven".to_string(),
            WordDigit::Eight   => "eight".to_string(),
            WordDigit::Nine    => "nine".to_string(),
        }
    }
}
impl WordDigit {
    fn int(&self) -> u8 {
        match *self {
            WordDigit::Zero    => 0,
            WordDigit::One     => 1,
            WordDigit::Two     => 2,
            WordDigit::Three   => 3,
            WordDigit::Four    => 4,
            WordDigit::Five    => 5,
            WordDigit::Six     => 6,
            WordDigit::Seven   => 7,
            WordDigit::Eight   => 8,
            WordDigit::Nine    => 9,
        }
    }
}
impl WordDigit {
    pub fn iterator() -> Iter<'static, WordDigit> {
        static WORD_DIGITS: [WordDigit; 10] = [
            WordDigit::Zero,
            WordDigit::One,
            WordDigit::Two,
            WordDigit::Three,
            WordDigit::Four,
            WordDigit::Five,
            WordDigit::Six,
            WordDigit::Seven,
            WordDigit::Eight,
            WordDigit::Nine,
        ];
        return WORD_DIGITS.iter();
    }
}

pub fn entry(input: &str) -> i32 {
    warn("moving the goal post");
    let lines = input.split('\n');

    let mut sum: u32 = 0;
    for line in lines {
        for c in line.chars() {
            if c.is_alphabetic() {
                // warn(&c.to_string());
                // info("first char is alpha");
                // line[0];
                match get_word(line) {
                    Ok(x) => {
                        // dbg!(x.string());
                        sum += x.int() as u32;
                        // errr(&x.string())
                    },
                    Err(_) => (),
                }
            }
            else {
                // warn(&c.to_string());
                // info("first char is NON alpha");
            }
        }
        
    }
    info(&sum.to_string());
    


    return 69;
}
fn get_word(line: &str) -> Result<WordDigit,()> {
    let line = line.as_bytes();
    // let buffer = Vec::new();
    let length = line.len();
    for i in 0..length {
        let line = &line[i..];
        for item in line.iter().enumerate() {
            let index = item.0;
            let _value = item.1;
            if index == 6 {
                return Err(());
            }
            let remaining_string = str::from_utf8(&line[..index]).unwrap();
            // dbg!(&remaining_string);
            match valid_word(remaining_string) {
                Ok(worddigit) => {
                    // errr(&worddigit.string());
                    return Ok(worddigit);
                },
                Err(_string_so_far) => {
                    ();
                }
            }
        }
    }

    return Err(());
}
fn valid_word(line: &str) -> Result<WordDigit,String> {
    for worddigit in WordDigit::iterator() {
        // info("checking");
        if line.to_lowercase() == worddigit.string() {
            // errr(&line.to_lowercase());
            // warn(&worddigit.string());
            // dbg!(line);
            // info("done!");
            return Ok(worddigit.clone());      
        }
    }

    return Err(line.to_string());
}
