use crate::util::get_lines;

enum MonkeyOperationType {
    Addition,
    Multiplication,
    Square,
}

struct Monkey {
    items: Vec<usize>,
    operation_type: MonkeyOperationType,
    operation_value: usize,
    test_divisible_by: usize,
    test_pass_index: usize,
    test_fail_index: usize,
    inspection_count: usize,
}

impl Monkey {
    fn parse_input(input: &str) -> Vec<Monkey> {
        let mut results: Vec<Monkey> = Vec::new();

        let lines = get_lines(input);
        let mut line_offset = 1;
        while line_offset <= lines.len() {
            // starting items
            let mut items: Vec<usize> = Vec::new();
            let starting_items_str = lines[line_offset].trim().split(':').last().unwrap();
            line_offset += 1;
            let starting_items = starting_items_str.trim().split(',').collect::<Vec<&str>>();
            for starting_item in starting_items {
                items.push(starting_item.trim().parse().unwrap());
            }

            // operation
            let operations_str = lines[line_offset].trim().split(':').last().unwrap();
            line_offset += 1;
            let operation_type_char = if operations_str.contains('*') {
                '*'
            } else {
                '+'
            };
            let operations_split = operations_str.split(operation_type_char).collect::<Vec<&str>>();
            let operation_value_str = operations_split.last().unwrap().trim();
            let operation_type = if operation_value_str == "old" {
                MonkeyOperationType::Square
            } else if operation_type_char == '+' {
                MonkeyOperationType::Addition
            } else {
                MonkeyOperationType::Multiplication
            };
            let operation_value: usize = if operation_value_str == "old" {
                0
            } else {
                operation_value_str.parse().unwrap()
            };

            // test
            let test_divisible_by: usize = lines[line_offset].split("by").last().unwrap().trim().parse().unwrap();
            line_offset += 1;
            let test_pass_index: usize = lines[line_offset].split("monkey").last().unwrap().trim().parse().unwrap();
            line_offset += 1;
            let test_fail_index: usize = lines[line_offset].split("monkey").last().unwrap().trim().parse().unwrap();
            line_offset += 1;

            results.push(Monkey {
                items,
                operation_type,
                operation_value,
                test_divisible_by,
                test_pass_index,
                test_fail_index,
                inspection_count: 0,
            });

            line_offset += 2;
        }

        results
    }

    fn execute(&mut self, relief_value: Option<usize>, mod_product: usize) -> Vec<(usize, usize)> {
        let mut results: Vec<(usize, usize)> = Vec::new();
        let mut item_index = 0;
        loop {
            if item_index >= self.items.len() {
                break;
            }
            let item = &mut self.items[item_index];

            // inspection by monkey
            *item = match self.operation_type {
                MonkeyOperationType::Addition => *item + self.operation_value,
                MonkeyOperationType::Multiplication => *item * self.operation_value,
                MonkeyOperationType::Square => *item * *item,
            };
            *item %= mod_product;
            self.inspection_count += 1;

            // relief it is not broken
            *item /= match relief_value {
                Some(x) => x,
                None => 1,
            };

            // test by monkey to throw or not
            let should_throw = *item % self.test_divisible_by == 0;
            let throw_target_index = if should_throw {
                self.test_pass_index
            } else {
                self.test_fail_index
            };
            results.push((throw_target_index, item.clone()));
            item_index += 1;
        }
        self.items.clear();
        results
    }
}

// used to keep values in check from overflowing (part 2 had 10k round and no relief divisor)
// if we multiply all test_divisible_by values together and take each result and use the modulus
// operator on it, we guarantee the value still works for all monkeys while keep its value lower
// to avoid overflow
fn get_mod_product(monkeys: &Vec<Monkey>) -> usize {
    let mut result = 1;
    for monkey in monkeys {
        result *= monkey.test_divisible_by;
    }
    result
}

fn execute_rounds(monkeys: &mut Vec<Monkey>, mod_product: usize, relief_value: Option<usize>, round_count: usize) {
    for _ in 0..round_count {
        let mut monkey_index = 0;
        while monkey_index < monkeys.len() {
            let monkey = &mut monkeys[monkey_index];
            let throws = monkey.execute(relief_value, mod_product);
            for throw in throws {
                monkeys[throw.0].items.push(throw.1);
            }

            monkey_index += 1;
        }
    }
}

fn get_result(monkeys: &Vec<Monkey>) -> usize {
    let mut inspection_counts: Vec<usize> = Vec::new();
    for monkey in monkeys {
        inspection_counts.push(monkey.inspection_count);
    }
    inspection_counts.sort_by(|a, b| b.cmp(a));
    inspection_counts[0] * inspection_counts[1]
}

pub fn answer1(input: &str) -> String {
    let mut monkeys = Monkey::parse_input(input);
    let mod_product = get_mod_product(&monkeys);
    execute_rounds(&mut monkeys, mod_product, Some(3), 20);
    let result = get_result(&monkeys);
    result.to_string()
}

pub fn answer2(input: &str) -> String {
    let mut monkeys = Monkey::parse_input(input);
    let mod_product = get_mod_product(&monkeys);
    execute_rounds(&mut monkeys, mod_product, None, 10_000);
    let result = get_result(&monkeys);
    result.to_string()
}