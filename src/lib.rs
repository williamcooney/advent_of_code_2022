mod util;
mod day01;
mod day02;
mod day03;

pub struct AdventOfCode<'a> {
    day: u32,
    input: &'a str,
}

impl<'a> AdventOfCode<'a> {
    pub fn new(day: u32, input: &str) -> AdventOfCode {
        AdventOfCode {
            day,
            input,
        }
    }

    pub fn answer1(&self) -> String {
        match self.day {
            3 => day03::answer1(self.input),
            2 => day02::answer1(self.input),
            1 => day01::answer1(self.input),
            _ => String::from(self.input),
        }
    }

    pub fn answer2(&self) -> String {
        match self.day {
            3 => day03::answer2(self.input),
            2 => day02::answer2(self.input),
            1 => day01::answer2(self.input),
            _ => String::from(self.input),
        }
    }
}