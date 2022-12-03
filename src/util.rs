pub fn get_lines(input: &str) -> Vec<&str> {
    let mut results = input.split("\n").collect::<Vec<&str>>();
    if input.chars().last() == Some('\n') {
        results.remove(results.len() - 1);
    }
    results
}