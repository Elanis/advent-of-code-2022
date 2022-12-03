#[cfg(test)]
mod defi03_tests {
    use aoc2022::defi03;

    #[test]
    fn defi03() {
        let source = String::from(
"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
        );


        assert_eq!(defi03::line_priority("vJrwpWtwJgWrhcsFMMfFFhFp"), 16);
        assert_eq!(defi03::line_priority("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 38);
        assert_eq!(defi03::line_priority("PmmdzqPrVvPwwTWBwg"), 42);
        assert_eq!(defi03::line_priority("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), 22);
        assert_eq!(defi03::line_priority("ttgJtRGJQctTZtZT"), 20);
        assert_eq!(defi03::line_priority("CrZsJsPPZsGzwwsLwLmpwMDw"), 19);

        let res = defi03::do_work(source.clone());
        assert_eq!(res, 157);
    }
}