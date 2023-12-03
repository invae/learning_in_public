#[cfg(test)]
use crate::part_1;
#[cfg(test)]
use crate::part_2;


// #[test]
// fn module_tree() {
//     part_1::entry("test");
//     part_2::entry("test");
// }
#[test]
fn logger_online() {
    let msg = "test";
    ezlogs::info(msg);
    ezlogs::warn(msg);
    ezlogs::errr(msg);
}
#[test]
fn len_in_substr() {
    let line = b"963......................542........734.....901...914..........843.............523..........818..................691.....833...";
    let value = part_1::get_len(line);
    assert_eq!(value, 3);
}
#[test]
fn valid_symbols() {
    assert!(part_1::valid_symbol(b'$'));
    assert!(part_1::valid_symbol(b'*'));
    assert!(part_1::valid_symbol(b'/'));
    assert!(part_1::valid_symbol(b'%'));
    assert!(part_1::valid_symbol(b'='));
    assert!(part_1::valid_symbol(b'&'));
    assert!(part_1::valid_symbol(b'@'));
    assert!(part_1::valid_symbol(b'#'));
    assert!(part_1::valid_symbol(b'+'));
    assert!(part_1::valid_symbol(b'-'));
}