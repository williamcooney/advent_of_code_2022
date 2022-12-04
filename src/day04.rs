use crate::util::get_lines;

struct LineValues {
    first_start: u32,
    first_end: u32,
    second_start: u32,
    second_end: u32,
}

impl LineValues {
    fn new(line: &str) -> LineValues {
        let split = line.trim().split(",").collect::<Vec<&str>>();
    
        let first = split[0];
        let first_split = first.trim().split("-").collect::<Vec<&str>>();
        let first_start: u32 = first_split[0].parse().expect("failed to parse number");
        let first_end: u32 = first_split[1].parse().expect("failed to parse number");
    
        let second = split[1];
        let second_split = second.trim().split("-").collect::<Vec<&str>>();
        let second_start: u32 = second_split[0].parse().expect("failed to parse number");
        let second_end: u32 = second_split[1].parse().expect("failed to parse number");
    
        LineValues {
            first_start,
            first_end,
            second_start,
            second_end,
        }
    }

    fn contains(&self) -> bool {
        self.first_start <= self.second_start && self.first_end >= self.second_end
        || self.second_start <= self.first_start && self.second_end >= self.first_end
    }

    fn overlaps(&self) -> bool {
        self.contains()
        || self.first_start <= self.second_start && self.first_end >= self.second_start
        || self.second_start <= self.first_start && self.second_end >= self.first_start
    }
}

pub fn answer1(input: &str) -> String {
    let lines = get_lines(input);
    let mut result = 0;
    for line in lines.iter() {
        let values = LineValues::new(line);
        if values.contains() {
            result += 1;
        }
    }
    result.to_string()
}

pub fn answer2(input: &str) -> String {
    let lines = get_lines(input);
    let mut result = 0;
    for line in lines.iter() {
        let values = LineValues::new(line);
        if values.overlaps() {
            result += 1
        }
    }
    result.to_string()
}