struct DataRecord {
    opponent: i32,
    player: i32,
}

impl DataRecord {
    fn new(opponent: i32, player: i32) -> DataRecord {
        DataRecord {
            opponent,
            player,
        }
    }
}

fn get_data(input: &str, is_part_two: bool) -> Vec<DataRecord> {
    let mut lines = input.split("\n").collect::<Vec<&str>>();
    if input.chars().last() == Some('\n') {
        lines.remove(lines.len() - 1);
    }

    let mut results: Vec<DataRecord> = Vec::new();
    for line in lines.iter() {
        let line_items = line.split(" ").collect::<Vec<&str>>();
        let first = line_items[0].trim();
        let second = line_items[1].trim();

        let opponent = match first {
            "A" => 1,
            "B" => 2,
            _ => 3,
        };

        let player = match second {
            "X" => if is_part_two {
                let current_result = opponent - 1;
                if current_result < 1 {
                    3
                } else {
                    current_result
                }
            } else {
                1
            },
            "Y" => if is_part_two {
                opponent
            } else {
                2
            },
            _ => if is_part_two {
                let current_result = opponent + 1;
                if current_result > 3 {
                    1
                } else {
                    current_result
                }
            } else {
                3
            },
        };

        results.push(DataRecord::new(opponent, player));
    }

    results
}

fn process_data(data: Vec<DataRecord>) -> i32 {
    let mut results = 0;
    for item in data.iter() {
        let diff = item.opponent - item.player;
        let score = match diff.abs() {
            0 => 3, // draw
            1 => if diff > 0 { 0 } else { 6 }, // winner but not rock vs scissors
            _ => if diff > 0 { 6 } else { 0 }, // winner but rock vs scissors
        };
        results += item.player + score;
    }
    results
}

pub fn answer1(input: &str) -> String {
    let data = get_data(input, false);
    let result = process_data(data);
    result.to_string()
}

pub fn answer2(input: &str) -> String {
    let data = get_data(input, true);
    let result = process_data(data);
    result.to_string()
}