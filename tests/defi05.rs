#[cfg(test)]
mod defi05_tests {
    use aoc2022::defi05;

    #[test]
    fn defi05() {
        let source = String::from(
"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
        );

        let res = defi05::do_work(source.clone());
        assert_eq!(res, "CMZ");
        let res = defi05::do_work_2(source.clone());
        assert_eq!(res, "MCD");
    }
}