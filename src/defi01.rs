pub fn do_work(input : String) -> u32 {
    let elves = input.split("\n\n");
    let mut max = 0;

    for (_, elf) in elves.enumerate() {
    	let calories = elf.split("\n");
    	let mut total = 0;

    	for (_, calorie) in calories.enumerate() {
    		total += calorie.parse::<u32>().unwrap();
    	}

    	if total > max {
    		max = total;
    	}
    }

    max
}