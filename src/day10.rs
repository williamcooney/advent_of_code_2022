use crate::util::get_lines;

struct VirtualMachine {
    cycle: isize,
    register: isize,
    signal_strength: isize,
    instructions: Vec<isize>,
    instructions_offset: usize,
    screen: Vec<bool>,
}

impl VirtualMachine {
    fn new(instructions: Vec<isize>) -> VirtualMachine {
        VirtualMachine {
            cycle: 0,
            register: 1,
            signal_strength: 0,
            instructions,
            instructions_offset: 0,
            screen: Vec::new(),
        }
    }

    fn execute(&mut self) -> bool {
        // begin cycle
        if self.instructions_offset >= self.instructions.len() {
            return false;
        }
        self.cycle += 1;

        // during cycle
        self.signal_strength = self.cycle * self.register;
        let screen_pos = (self.cycle - 1) % 40;
        let pixel_is_lit = screen_pos >= self.register - 1 && screen_pos <= self.register + 1;
        self.screen.push(pixel_is_lit);

        // end cycle
        self.register += self.instructions[self.instructions_offset];
        self.instructions_offset += 1;
        true
    }

    fn print_screen(&self)-> String {
        let mut result = String::new();
        let mut offset = 0;
        for _ in 0..6 {
            for _ in 0..40 {
                let value = if self.screen[offset] {
                    '#'
                } else {
                    '.'
                };
                offset += 1;
                result.push(value);
            }
            result.push('\n');
        }
        result
    }
}

fn get_data(input: &str) -> Vec<isize> {
    let mut results: Vec<isize> = Vec::new();
    let lines = get_lines(input);
    for line in lines {
        results.push(0); // noop is 1 cycle of nothing and addx first cycle is also nothing
        if line.trim() != "noop" { // addx only other command and second cycle value parsing
            let value: isize = line.trim().split(' ').last().unwrap().parse().unwrap();
            results.push(value);
        }
    }
    results
}

pub fn answer1(input: &str) -> String {
    let data = get_data(input);
    let mut vm = VirtualMachine::new(data);

    let mut result = 0;
    loop {
        let did_execute = vm.execute();
        if (vm.cycle - 20) % 40 == 0 {
            result += vm.signal_strength;
        }

        if !did_execute {
            break;
        }
    }

    result.to_string()
}

pub fn answer2(input: &str) -> String {
    let data = get_data(input);
    let mut vm = VirtualMachine::new(data);

    loop {
        let did_execute = vm.execute();
        if !did_execute {
            break;
        }
    }

    vm.print_screen()
}