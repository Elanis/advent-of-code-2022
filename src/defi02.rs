pub fn line_score(line : &str) -> u32 {
    let choices : Vec<&str> = line.split(' ').collect();

    let choice_score = match choices[1] {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
         &_ => todo!()
    };

    // Draw
    let mut win_score = 0;
    if (choices[0] == "A" && choices[1] == "X") ||
        (choices[0] == "B" && choices[1] == "Y") ||
        (choices[0] == "C" && choices[1] == "Z") {

        win_score = 3;
    }

    if (choices[0] == "A" && choices[1] == "Y") ||
        (choices[0] == "B" && choices[1] == "Z") ||
        (choices[0] == "C" && choices[1] == "X") {

        win_score = 6;
    }

    win_score + choice_score
}

pub fn do_work(input : String) -> u32 {
    let lines = input.split("\n");
    let mut score = 0;

    for (_, line) in lines.enumerate() {
    	score += line_score(line);
    }

    score
}

pub fn line_score_2(line : &str) -> u32 {
    let choices : Vec<&str> = line.split(' ').collect();

    let mut choosen_one = "";
    if choices[1] == "X" { // Loose
        choosen_one = match choices[0] {
            "A" => "Z",
            "B" => "X",
            "C" => "Y",
            &_ => todo!()
        }
    }
    if choices[1] == "Y" { // Draw
        choosen_one = match choices[0] {
            "A" => "X",
            "B" => "Y",
            "C" => "Z",
            &_ => todo!()
        }
    }
    if choices[1] == "Z" { // Win
        choosen_one = match choices[0] {
            "A" => "Y",
            "B" => "Z",
            "C" => "X",
            &_ => todo!()
        }
    }

    line_score(&(choices[0].to_owned() + " " + choosen_one))
}

pub fn do_work_2(input : String) -> u32 {
    let lines = input.split("\n");
    let mut score = 0;

    for (_, line) in lines.enumerate() {
        score += line_score_2(line);
    }

    score
}