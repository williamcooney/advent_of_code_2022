use std::fs;

use advent_of_code_2022::AdventOfCode;

fn main() {
    let mut day = 1;
    let mut did_fail = false;
    while day <= 25 {
        let input_path = build_path("data/", day, "/input.txt");
        let answer1_path = build_path("data/", day, "/answer1.txt");
        let answer2_path = build_path("data/", day, "/answer2.txt");

        let input = fs::read_to_string(input_path).expect("unable to read input");
        let answer1 = fs::read_to_string(answer1_path).expect("unable to read answer1");
        let answer2 = fs::read_to_string(answer2_path).expect("unable to read answer2");

        let instance = AdventOfCode::new(day, &input);
        let result1 = instance.answer1();
        let result2 = instance.answer2();

        let output_dir = build_path("dist/", day, "");
        let output1_path = build_path("dist/", day, "/output1.txt");
        let output2_path = build_path("dist/", day, "/output2.txt");

        fs::create_dir_all(&output_dir).expect("failed to create output directory");
        fs::write(&output1_path, &result1).expect("unable to write output1");
        fs::write(&output2_path, &result2).expect("unable to write output2");

        if answer1.trim() != result1 {
            println!("[DAY {}][1] FAILED => EXPECTED {} GOT {}", day, answer1, result1);
            did_fail = true;
        }

        if answer2.trim() != result2 {
            println!("[DAY {}][2] FAILED => EXPECTED {} GOT {}", day, answer2, result2);
            did_fail = true;
        }

        day += 1;
    }

    if !did_fail {
        println!("✔ EVERYTHING PASSED");
    }
}

fn build_path(prefix: &str, day: u32, file_name: &str) -> String {
    let mut day_path = String::from(prefix);
    if day < 10 {
        day_path.push_str("0");
    }
    day_path.push_str(&day.to_string());
    day_path.push_str(&file_name);
    return day_path;
}
