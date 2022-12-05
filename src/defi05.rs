fn parse_cranes(lines_str : &str) -> Vec<Vec<String>> {
    let mut lines : Vec<&str> = lines_str.split('\n').collect();
    lines.pop();
    let size = (lines[0].len() + 1) / 4;
    let mut data : Vec<Vec<String>> = Vec::new();

    for _i in 0..size {
        data.push(Vec::new());
    }

    for line in lines {
        for i in 0..line.len() {
            if i % 4 != 0 {
                continue;
            }

            let item: String = line.chars().skip(i + 1).take(1).collect();
            if item != " " {
                data[(i / 4)].push(item)
            }
        }
    }

    for i in 0..size {
        data[i].reverse();
    }

    data
}

fn apply_moves(mut initial : Vec<Vec<String>>, orders : Vec<&str>) -> Vec<Vec<String>> {
    for order in orders {
        let parts = order.split(' ').collect::<Vec<&str>>();

        let amount = parts[1].parse::<u32>().unwrap();
        let from = parts[3].parse::<usize>().unwrap();     
        let to = parts[5].parse::<usize>().unwrap();     
    
        for _i in 0..amount {
            let item = initial[from - 1].pop().unwrap();
            initial[to - 1].push(item);
        }
    }

    initial
}

fn show_top_line(data : Vec<Vec<String>>) -> String {
    let mut final_string : String = "".to_owned();

    for col in data {
        final_string.push_str(&col[col.len() - 1].to_owned());
    }

    final_string
}

pub fn do_work(input : String) -> String {
    let parts : Vec<&str> = input.split("\n\n").collect();
    let initial : Vec<Vec<String>> = parse_cranes(parts[0]);
    let orders : Vec<&str> = parts[1].split('\n').collect();

    let result = apply_moves(initial, orders);

    show_top_line(result)
}