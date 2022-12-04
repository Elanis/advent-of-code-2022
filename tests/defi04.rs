#[cfg(test)]
mod defi04_tests {
    use aoc2022::defi04;

    #[test]
    fn defi04() {
        let source = String::from(
"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
        );

        let res = defi04::do_work(source.clone());
        assert_eq!(res, 2);
    }

    #[test]
    fn defi04_2() {
        let source = String::from(
"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
        );

        let res = defi04::do_work_2(source.clone());
        assert_eq!(res, 4);
    }
}