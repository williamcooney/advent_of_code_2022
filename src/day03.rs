use crate::util::get_lines;

fn get_value_for_char(c: &char) -> u32 {
    let value = if *c >= 'a' && *c <= 'z' {
        *c as u32 - 'a' as u32 + 1
    } else {
        *c as u32 - 'A' as u32 + 27
    };
    value
}

pub fn answer1(input: &str) -> String {
    let lines = get_lines(input);

    let mut result = 0;
    for line in lines.iter() {
        let trimmed = line.trim();
        let first = &trimmed[..trimmed.len() / 2];
        let second = &trimmed[trimmed.len() / 2..trimmed.len()];
        
        let mut chars: Vec<char> = Vec::new();
        for c in first.chars() {
            for d in second.chars() {
                if c == d && !chars.contains(&c) {
                    chars.push(c);
                }
            }
        }

        for c in chars.iter() {
            result += get_value_for_char(c);
        }
    }

    result.to_string()
}

pub fn answer2(input: &str) -> String {
    let lines = get_lines(input);

    let mut result = 0;
    let mut line_index = 0;
    while line_index < lines.len() {
        let line1 = lines[line_index].trim();
        let line2 = lines[line_index + 1].trim();
        let line3 = lines[line_index + 2].trim();

        let mut found = false;
        for char1 in line1.chars() {
            for char2 in line2.chars() {
                if char1 != char2 {
                    continue;
                }
                for char3 in line3.chars() {
                    if char1 == char3 {
                        result += get_value_for_char(&char1);
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
            if found {
                break;
            }
        }

        line_index += 3;
    }

    result.to_string()
}