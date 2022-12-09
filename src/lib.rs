mod util;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

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
            9 => day09::answer1(self.input),
            8 => day08::answer1(self.input),
            7 => day07::answer1(self.input),
            6 => day06::answer1(self.input),
            5 => day05::answer1(self.input),
            4 => day04::answer1(self.input),
            3 => day03::answer1(self.input),
            2 => day02::answer1(self.input),
            1 => day01::answer1(self.input),
            _ => String::from(self.input),
        }
    }

    pub fn answer2(&self) -> String {
        match self.day {
            9 => day09::answer2(self.input),
            8 => day08::answer2(self.input),
            7 => day07::answer2(self.input),
            6 => day06::answer2(self.input),
            5 => day05::answer2(self.input),
            4 => day04::answer2(self.input),
            3 => day03::answer2(self.input),
            2 => day02::answer2(self.input),
            1 => day01::answer2(self.input),
            _ => String::from(self.input),
        }
    }
}