pub fn range_include_other(range1 : &str, range2: &str) -> bool {
    let r1values : Vec<&str> = range1.split('-').collect();
    let r1_1 = r1values[0].parse::<i32>().unwrap();
    let r1_2 = r1values[1].parse::<i32>().unwrap();

    let r2values : Vec<&str> = range2.split('-').collect();
    let r2_1 = r2values[0].parse::<i32>().unwrap();
    let r2_2 = r2values[1].parse::<i32>().unwrap();

    if
        (r1_1 <= r2_1 && r1_2 >= r2_2) ||
        (r2_1 <= r1_1 && r2_2 >= r1_2) {

            return true;
    }

    false
}


pub fn do_work(input : String) -> u32 {
    let lines = input.split("\n");
    let mut score = 0;

    for (_, line) in lines.enumerate() {
        let ranges : Vec<&str> = line.split(',').collect();

        if range_include_other(ranges[0], ranges[1]) {
    	   score += 1;
        }
    }

    score
}

pub fn range_partially_include_other(range1 : &str, range2: &str) -> bool {
    let r1values : Vec<&str> = range1.split('-').collect();
    let r1_1 = r1values[0].parse::<i32>().unwrap();
    let r1_2 = r1values[1].parse::<i32>().unwrap();

    let r2values : Vec<&str> = range2.split('-').collect();
    let r2_1 = r2values[0].parse::<i32>().unwrap();
    let r2_2 = r2values[1].parse::<i32>().unwrap();

    if
        (r1_1 <= r2_1 && r1_2 >= r2_1) ||
        (r1_1 <= r2_2 && r1_2 >= r2_2) ||
        (r2_1 <= r1_1 && r2_2 >= r1_1) ||
        (r2_1 <= r1_2 && r2_2 >= r1_2) {

            return true;
    }

    false
}

pub fn do_work_2(input : String) -> u32 {
    let lines = input.split("\n");
    let mut score = 0;

    for (_, line) in lines.enumerate() {
        let ranges : Vec<&str> = line.split(',').collect();

        if range_partially_include_other(ranges[0], ranges[1]) {
           score += 1;
        }
    }

    score
}