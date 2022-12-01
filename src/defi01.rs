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

pub fn do_work_2(input : String) -> u32 {
    let elves = input.split("\n\n");
    let mut values = Vec::new();

    for (_, elf) in elves.enumerate() {
    	let calories = elf.split("\n");
    	let mut total = 0;

    	for (_, calorie) in calories.enumerate() {
    		total += calorie.parse::<u32>().unwrap();
    	}

    	values.push(total);
    }

    values.sort_by(|a, b| b.cmp(a));

    values[0] + values[1] + values[2]
}