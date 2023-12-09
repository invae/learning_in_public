#[cfg(test)]
use crate::part_1;
#[cfg(test)]
use crate::part_2;


#[test]
fn module_tree() {
    part_1::entry("test");
    part_2::entry("test");
}
#[test]
fn logger_online() {
    let msg = "test";
    ezlogs::info(msg);
    ezlogs::warn(msg);
    ezlogs::errr(msg);
}