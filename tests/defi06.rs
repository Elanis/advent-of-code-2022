#[cfg(test)]
mod defi06_tests {
    use aoc2022::defi06;

    #[test]
    fn defi06() {
        assert_eq!(defi06::do_work("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()), 7);
        assert_eq!(defi06::do_work("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 5);
        assert_eq!(defi06::do_work("nppdvjthqldpwncqszvftbrmjlhg".to_string()), 6);
        assert_eq!(defi06::do_work("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()), 10);
        assert_eq!(defi06::do_work("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()), 11);
    }
}