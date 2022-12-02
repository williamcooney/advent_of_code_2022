fn get_data(input: &str) -> Vec<u32> {
    let mut lines = input.split("\n").collect::<Vec<&str>>();
    if input.chars().last() == Some('\n') {
        lines.remove(lines.len() - 1);
    }

    let mut results: Vec<u32> = vec![0];
    let mut index = 0;
    for line in lines.iter() {
        if line.trim().len() <= 0 {
            index += 1;
            results.push(0);
        } else {
            let value: u32 = line.trim().parse().unwrap();
            results[index] += value;
        }
    }

    return results;
}

pub fn answer1(input: &str) -> String {
    let data = get_data(input);

    let mut max = 0;
    for item in data.iter() {
        if *item > max {
            max = *item;
        }
    }

    max.to_string()
}

pub fn answer2(input: &str) -> String {
    let mut data = get_data(input);
    data.sort_by(|a, b| b.cmp(a));

    let mut result = 0;
    for index in 0..3 {
        let item = data.get(index);
        if let Some(value) = item {
            result += value;
        }
    }

    result.to_string()
}