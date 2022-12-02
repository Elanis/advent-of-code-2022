#[cfg(test)]
mod defi02_tests {
    use aoc2022::defi02;

    #[test]
    fn defi02() {
        let source = String::from(
"A Y
B X
C Z"
        );


        assert_eq!(defi02::line_score("A Y"), 8);
        assert_eq!(defi02::line_score("B X"), 1);
        assert_eq!(defi02::line_score("C Z"), 6);

        let res = defi02::do_work(source.clone());
        assert_eq!(res, 15);
    }

    #[test]
    fn defi02_02() {
        let source = String::from(
"A Y
B X
C Z"
        );


        assert_eq!(defi02::line_score_2("A Y"), 4);
        assert_eq!(defi02::line_score_2("B X"), 1);
        assert_eq!(defi02::line_score_2("C Z"), 7);

        let res = defi02::do_work_2(source.clone());
        assert_eq!(res, 12);
    }
}