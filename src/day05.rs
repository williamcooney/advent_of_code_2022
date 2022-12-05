use crate::util::get_lines;

struct Instruction {
    source: usize,
    destination: usize,
    count: u32,
}

impl Instruction {
    fn new(source: usize, destination: usize, count: u32) -> Instruction {
        Instruction {
            source,
            destination,
            count,
        }
    }
}

struct Columns {
    values: Vec<Vec<char>>,
}

impl Columns {
    fn new(values: Vec<Vec<char>>) -> Columns {
        Columns {
            values,
        }
    }

    fn execute_9000(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.count {
            let value = self.values[instruction.source - 1].pop().unwrap();
            self.values[instruction.destination - 1].push(value);
        }
    }

    fn execute_9001(&mut self, instruction: &Instruction) {
        let mut values: Vec<char> = Vec::new();
        for _ in 0..instruction.count {
            let value = self.values[instruction.source - 1].pop().unwrap();
            values.push(value);
        }
        for value in values.iter().rev() {
            self.values[instruction.destination - 1].push(*value);
        }
    }

    fn output(&self) -> String {
        let mut result = String::new();
        for value in self.values.iter() {
            if value.len() > 0 {
                result.push(*value.last().unwrap());
            }
        }
        result
    }
}

struct Data {
    columns: Columns,
    instructions: Vec<Instruction>,
}

impl Data {
    fn new(columns: Columns, instructions: Vec<Instruction>) -> Data {
        Data {
            columns,
            instructions,
        }
    }
}

fn get_data(input: &str) -> Data {
    let lines = get_lines(input);

    let mut crate_lines: Vec<&str> = Vec::new();
    let mut instruction_lines: Vec<&str> = Vec::new();
    for line in lines.iter() {
        let trimmed = line.trim();
        if trimmed.chars().next() == Some('[') {
            crate_lines.push(line);
        } else if trimmed.chars().next() == Some('m') {
            instruction_lines.push(line);
        }
    }

    let mut columns: Vec<Vec<char>> = Vec::new();
    let column_count = (crate_lines.last().unwrap().len() + 1) / 4;
    for _ in 0..column_count {
        columns.push(Vec::new());
    }
    for line in crate_lines.iter().rev() {
        let mut offset = 0;
        let mut column_index = 0;
        while offset + 4 <= line.len() {
            let wrapped_value = &line[offset..offset + 4];
            if wrapped_value.chars().next() == Some('[') {
                let value = wrapped_value.chars().nth(1).unwrap();
                columns[column_index].push(value);
            }
            offset += 4;
            column_index += 1;
        }
    }
    let columns = Columns::new(columns);

    let mut instructions: Vec<Instruction> = Vec::new();
    for line in instruction_lines {
        let mut split1 = line.split("move ").nth(1).unwrap().split(" from ");
        let count: u32 = split1.next().unwrap().trim().parse().unwrap();
        let mut split2 = split1.next().unwrap().split(" to ");
        let source: usize = split2.next().unwrap().trim().parse().unwrap();
        let destination: usize = split2.next().unwrap().trim().parse().unwrap();
        instructions.push(Instruction::new(source, destination, count));
    }

    Data::new(columns, instructions)
}

pub fn answer1(input: &str) -> String {
    let mut data = get_data(input);
    for instruction in data.instructions {
        data.columns.execute_9000(&instruction);
    }
    data.columns.output()
}

pub fn answer2(input: &str) -> String {
    let mut data = get_data(input);
    for instruction in data.instructions {
        data.columns.execute_9001(&instruction);
    }
    data.columns.output()
}