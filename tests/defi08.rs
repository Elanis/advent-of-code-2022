#[cfg(test)]
mod defi08_tests {
    use aoc2022::defi08;

    #[test]
    fn defi08() {
        let input = "30373
25512
65332
33549
35390".to_string();

        assert_eq!(defi08::do_work(input.clone()), 21);

        let map = defi08::parse(input.clone());

        assert_eq!(defi08::is_visible(map.clone(), 1, 1), true);
        assert_eq!(defi08::is_visible(map.clone(), 1, 2), true);
        assert_eq!(defi08::is_visible(map.clone(), 1, 3), false);
        assert_eq!(defi08::is_visible(map.clone(), 2, 1), true);
        assert_eq!(defi08::is_visible(map.clone(), 2, 2), false);
        assert_eq!(defi08::is_visible(map.clone(), 2, 3), true);
        assert_eq!(defi08::is_visible(map.clone(), 3, 1), false);
        assert_eq!(defi08::is_visible(map.clone(), 3, 2), true);
        assert_eq!(defi08::is_visible(map.clone(), 3, 3), false);


        assert_eq!(defi08::scenic_score(map.clone(), 1, 2), 4);
        assert_eq!(defi08::scenic_score(map.clone(), 3, 2), 8);

        assert_eq!(defi08::do_work_2(input.clone()), 8);
    }
}