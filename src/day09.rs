use crate::util::get_lines;

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position {
            x,
            y,
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct Rope {
    knots: Vec<Position>,
    last_knot_visits: Vec<Position>,
}

impl Rope {
    fn new(tail_count: u32) -> Rope {
        let mut knots: Vec<Position> = Vec::new();
        for _ in 0..tail_count {
            knots.push(Position::new(0,0));
        }

        Rope {
            knots,
            last_knot_visits: vec![Position::new(0, 0)],
        }
    }

    fn advance(&mut self, direction: char) {
        self.advance_head(direction);
        for index in 1..self.knots.len() {
            self.advance_trailing_knot_if_needed(index);
        }
        self.update_tail_visits();
    }

    fn advance_head(&mut self, direction: char) {
        match direction {
            'U' => self.knots[0].y += 1,
            'D' => self.knots[0].y -= 1,
            'R' => self.knots[0].x += 1,
            'L' => self.knots[0].x -= 1,
            _ => panic!(),
        }
    }

    fn advance_trailing_knot_if_needed(&mut self, index: usize) {
        let x_diff = (self.knots[index - 1].x - self.knots[index].x).abs();
        let y_diff = (self.knots[index - 1].y - self.knots[index].y).abs();
        if x_diff <= 1 && y_diff <= 1 {
            return;
        }

        if x_diff == 0 { // vert
            if self.knots[index - 1].y > self.knots[index].y {
                self.knots[index].y += 1;
            } else {
                self.knots[index].y -= 1;
            }
        } else if y_diff == 0 { // horiz
            if self.knots[index - 1].x > self.knots[index].x {
                self.knots[index].x += 1;
            } else {
                self.knots[index].x -= 1;
            }
        } else { // diag
            if self.knots[index - 1].x > self.knots[index].x {
                self.knots[index].x += 1;
            } else {
                self.knots[index].x -= 1;
            }
            if self.knots[index - 1].y > self.knots[index].y {
                self.knots[index].y += 1;
            } else {
                self.knots[index].y -= 1;
            }
        }
    }

    fn update_tail_visits(&mut self) {
        let last_knot = &self.knots.last().unwrap();
        let new_position = Position::new(last_knot.x, last_knot.y);
        for position in &self.last_knot_visits {
            if new_position == *position {
                return;
            }
        }
        self.last_knot_visits.push(new_position);
    }
}

fn get_instructions(input: &str) -> Vec<char> {
    let mut results: Vec<char> = Vec::new();
    let lines = get_lines(input);
    for line in lines {
        let mut split = line.trim().split(' ');
        let direction = split.next().unwrap().chars().next().unwrap();
        let count: u32 = split.next().unwrap().trim().parse().unwrap();
        for _ in 0..count {
            results.push(direction);
        }
    }
    results
}

pub fn answer1(input: &str) -> String {
    let instructions = get_instructions(input);
    let mut rope = Rope::new(2);

    for instruction in instructions {
        rope.advance(instruction);
    }

    rope.last_knot_visits.len().to_string()
}

pub fn answer2(input: &str) -> String {
    let instructions = get_instructions(input);
    let mut rope = Rope::new(10);

    for instruction in instructions {
        rope.advance(instruction);
    }

    rope.last_knot_visits.len().to_string()
}