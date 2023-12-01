mod constants;
use crate::constants::{
    NO_COLOR,
    LIGHT_BLUE,
    YELLOW,
    RED,
};

pub fn info(msg: &str) {
    eprintln!("[{}{}{}] {}", LIGHT_BLUE, "info", NO_COLOR, msg);
}
pub fn warn(msg: &str) {
    eprintln!("[{}{}{}] {}", YELLOW, "warn", NO_COLOR, msg);
}
pub fn errr(msg: &str) {
    eprintln!("[{}{}{}] {}", RED, "errr", NO_COLOR, msg);
}
