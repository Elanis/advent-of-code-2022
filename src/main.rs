use std::fs;
use aoc2022::*;

fn main() {
    let defi01_content = fs::read_to_string("data/01.txt")
        .expect("Should have been able to read the file");
    println!("01 (1/2): {:?}", defi01::do_work(defi01_content.clone()));
    println!("01 (2/2): {:?}", defi01::do_work_2(defi01_content.clone()));

    let defi02_content = fs::read_to_string("data/02.txt")
        .expect("Should have been able to read the file");
    println!("02 (1/2): {:?}", defi02::do_work(defi02_content.clone()));
    println!("02 (2/2): {:?}", defi02::do_work_2(defi02_content.clone()));

    let defi03_content = fs::read_to_string("data/03.txt")
        .expect("Should have been able to read the file");
    println!("03 (1/2): {:?}", defi03::do_work(defi03_content.clone()));
    println!("03 (2/2): {:?}", defi03::do_work_2(defi03_content.clone()));

    let defi04_content = fs::read_to_string("data/04.txt")
        .expect("Should have been able to read the file");
    println!("04 (1/2): {:?}", defi04::do_work(defi04_content.clone()));
    println!("04 (2/2): {:?}", defi04::do_work_2(defi04_content.clone()));

    let defi05_content = fs::read_to_string("data/05.txt")
        .expect("Should have been able to read the file");
    println!("05 (1/2): {:?}", defi05::do_work(defi05_content.clone()));
    println!("05 (2/2): {:?}", defi05::do_work_2(defi05_content.clone()));

    let defi06_content = fs::read_to_string("data/06.txt")
        .expect("Should have been able to read the file");
    println!("06 (1/2): {:?}", defi06::do_work(defi06_content.clone()));
    println!("06 (2/2): {:?}", defi06::do_work_2(defi06_content.clone()));

    let defi07_content = fs::read_to_string("data/07.txt")
        .expect("Should have been able to read the file");
    println!("07 (1/2): {:?}", defi07::do_work(defi07_content.clone()));
    println!("07 (2/2): {:?}", defi07::do_work_2(defi07_content.clone()));

    let defi08_content = fs::read_to_string("data/08.txt")
        .expect("Should have been able to read the file");
    println!("08 (1/2): {:?}", defi08::do_work(defi08_content.clone()));
    //println!("08 (2/2): {:?}", defi08::do_work_2(defi08_content.clone()));
}
