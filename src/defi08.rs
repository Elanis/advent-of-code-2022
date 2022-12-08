pub fn is_visible(grid : Vec<Vec<u32>>, y : usize, x : usize) -> bool {
    if x == 0 || y == 0 {
        return true;
    }

    // Left
    let mut visible = true;
    for i in 0..x {
        if grid[y][i] >= grid[y][x] {
            visible = false;
            break;
        }
    }

    if visible {
        return true;
    }

    // Right
    visible = true;
    for i in (x + 1)..grid[y].len() {
        if grid[y][i] >= grid[y][x] {
            visible = false;
            break;
        }
    }

    if visible {
        return true;
    }

    // Top
    visible = true;
    for i in 0..y {
        if grid[i][x] >= grid[y][x] {
            visible = false;
            break;
        }
    }

    if visible {
        return true;
    }

    // Bottom
    visible = true;
    for i in (y + 1)..grid.len() {
        if grid[i][x] >= grid[y][x] {
            visible = false;
            break;
        }
    }

    if visible {
        return true;
    }

    false
}

pub fn parse(input : String) -> Vec<Vec<u32>> {
    let mut res : Vec<Vec<u32>> = Vec::new();
    for (i, line) in input.split('\n').enumerate() {
        res.push(Vec::new());

        for (_, col) in line.chars().enumerate() {
            res[i].push(
                col.to_digit(10).unwrap()
            );
        }
    }

    res
}

pub fn do_work(input : String) -> u32 {
    let grid = parse(input);

    let mut total = 0; 
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if is_visible(grid.clone(), y, x) {
                total += 1;
            }
        }
    }

    total
}