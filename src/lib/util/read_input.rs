use std::fs;

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect(&format!("Something went wrong reading file: {}", path))
}

pub fn read_input(day: &str) -> String {
    read_file(&format!("./inputs/{}.txt", day))
}
