use std::collections::VecDeque;

fn has_dupes(chars: &VecDeque<char>) -> bool {
    for index in 0..chars.len() {
        let first = chars.get(index).unwrap();
        for index2 in (index + 1)..chars.len() {
            let second = chars.get(index2).unwrap();
            if first == second {
                return true;
            }
        }
    }
    false
}

pub fn answer1(input: &str) -> String {
    let mut chars = input.trim().chars();
    let mut running_chars: VecDeque<char> = VecDeque::new();
    for _ in 0..4 {
        running_chars.push_back(chars.next().unwrap());
    }

    let mut count = 4;
    loop {
        if !has_dupes(&running_chars) {
            break;
        }

        count += 1;
        running_chars.pop_front();
        running_chars.push_back(chars.next().unwrap());
    }

    count.to_string()
}

pub fn answer2(input: &str) -> String {
    let mut chars = input.trim().chars();
    let mut running_chars: VecDeque<char> = VecDeque::new();
    for _ in 0..14 {
        running_chars.push_back(chars.next().unwrap());
    }

    let mut count = 14;
    loop {
        if !has_dupes(&running_chars) {
            break;
        }

        count += 1;
        running_chars.pop_front();
        running_chars.push_back(chars.next().unwrap());
    }

    count.to_string()
}