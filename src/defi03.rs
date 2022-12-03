fn letter_score(letter : char) -> u32 {
    if letter >= 'a' && letter <= 'z' {
        return letter as u32 - 'a' as u32 + 1;
    }

    letter as u32 - 'A' as u32 + 27
}

pub fn line_priority(line : &str) -> u32 {
    let letters : Vec<char> = line.chars().collect();
    let mut first_compartment = Vec::new();

    for i in 0..(letters.len() / 2) {
        first_compartment.push(letters[i]);
    }

    for i in (letters.len() / 2)..letters.len() {
        if first_compartment.contains(&letters[i]) {
            return letter_score(letters[i]);
        }
    }

    0
}

pub fn do_work(input : String) -> u32 {
    let lines = input.split("\n");
    let mut score = 0;

    for (_, line) in lines.enumerate() {
    	score += line_priority(line);
    }

    score
}

pub fn common_item(line1 : &str, line2 : &str, line3 : &str) -> u32 {
    let letters1 : Vec<char> = line1.chars().collect();
    let letters2 : Vec<char> = line2.chars().collect();
    let letters3 : Vec<char> = line3.chars().collect();

    for i in 0..letters1.len() {
        if letters2.contains(&letters1[i]) && letters3.contains(&letters1[i]) {
            return letter_score(letters1[i]);
        }
    }

    0
}

pub fn do_work_2(input : String) -> u32 {
    let lines : Vec<&str> = input.split("\n").collect();
    let mut score = 0;

    for i in 0..lines.len() {
        if i % 3 == 2 {
            score += common_item(lines[i - 2], lines[i - 1], lines[i]);
        }
    }

    score
}