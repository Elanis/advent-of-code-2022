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