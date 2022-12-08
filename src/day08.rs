use crate::util::get_lines;

fn get_data(input: &str) -> Vec<Vec<u32>> {
    let lines = get_lines(input);
    let mut results: Vec<Vec<u32>> = Vec::new();
    for line in lines.iter() {
        let mut row: Vec<u32> = Vec::new();
        for c in line.trim().chars() {
            let value: u32 = c.to_digit(10).unwrap();
            row.push(value);
        }
        results.push(row);
    }
    results
}

fn is_visible(grid: &Vec<Vec<u32>>, row_index: usize, col_index: usize) -> bool {
    let row = &grid[row_index];
    let value = &row[col_index];

    let is_blocked = |row_index2: usize, col_index2: usize| {
        grid[row_index2][col_index2] >= *value
    };

    // check left
    let mut result = true;
    for i in 0..col_index {
        if is_blocked(row_index, i) {
            result = false;
        }
    }
    if result {
        return true;
    }

    // check right
    result = true;
    for i in (col_index + 1)..row.len() {
        if is_blocked(row_index, i)  {
            result = false;
        }
    }
    if result {
        return true;
    }

    // check top
    result = true;
    for i in 0..row_index {
        if is_blocked(i, col_index) {
            result = false;
        }
    }
    if result {
        return true;
    }

    // check bottom
    result = true;
    for i in (row_index + 1)..grid.len() {
        if is_blocked(i, col_index) {
            result = false;
        }
    }
    if result {
        return true;
    }

    false
}

fn get_scenic_score(grid: &Vec<Vec<u32>>, row_index: usize, col_index: usize) -> u32 {
    let row = &grid[row_index];
    let value = &row[col_index];
    
    let is_blocked = |row_index2: usize, col_index2: usize| {
        grid[row_index2][col_index2] >= *value
    };

    let mut result = 1;

    // check left
    let mut current_result = 0;
    for i in (0..col_index).rev() {
        current_result += 1;
        if is_blocked(row_index, i) {
            break;
        }
    }
    if current_result == 0 {
        return 0;
    }
    result *= current_result;

    // check right
    current_result = 0;
    for i in (col_index + 1)..row.len() {
        current_result += 1;
        if is_blocked(row_index, i) {
            break;
        }
    }
    if current_result == 0 {
        return 0;
    }
    result *= current_result;

    // check top
    current_result = 0;
    for i in (0..row_index).rev() {
        current_result += 1;
        if is_blocked(i, col_index) {
            break;
        }
    }
    if current_result == 0 {
        return 0;
    }
    result *= current_result;

    // check bottom
    current_result = 0;
    for i in (row_index + 1)..grid.len() {
        current_result += 1;
        if is_blocked(i, col_index) {
            break;
        }
    }
    if current_result == 0 {
        return 0;
    }
    result *= current_result;

    result
}

pub fn answer1(input: &str) -> String {
    let grid = get_data(input);

    let mut result = 0;
    let mut row_index = 0;
    while row_index < grid.len() {
        let row = &grid[row_index];

        let mut col_index = 0;
        while col_index < row.len() {
            if is_visible(&grid, row_index, col_index) {
                result += 1;
            }

            col_index += 1
        }

        row_index += 1;
    }

    result.to_string()
}

pub fn answer2(input: &str) -> String {
    let grid = get_data(input);

    let mut result = 0;
    let mut row_index = 0;
    while row_index < grid.len() {
        let row = &grid[row_index];

        let mut col_index = 0;
        while col_index < row.len() {
            let score = get_scenic_score(&grid, row_index, col_index);
            if score > result {
                result = score;
            }

            col_index += 1
        }

        row_index += 1;
    }

    result.to_string()
}