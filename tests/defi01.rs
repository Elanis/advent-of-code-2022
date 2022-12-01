#[cfg(test)]
mod defi01_tests {
    use aoc2022::defi01;

    #[test]
    fn defi01() {
        let res = defi01::do_work(String::from(
"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
        ));

        assert_eq!(res, 24000);
    }
}