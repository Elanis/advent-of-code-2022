use std::fs;
use aoc2022::*;

fn main() {
    let defi01_content = fs::read_to_string("data/01.txt")
        .expect("Should have been able to read the file");
    println!("01 (1/2): {:?}", defi01::do_work(defi01_content.clone()));
    println!("01 (2/2): {:?}", defi01::do_work_2(defi01_content.clone()));
}
