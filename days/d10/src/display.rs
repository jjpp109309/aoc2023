use std::fs;
use color_print::cprint;

pub fn print_path(path: &str, loop_path: &Vec<String>) {
    if let Ok(file) = fs::read_to_string(path) {
        for (row, line) in file.lines().enumerate() {
            for (col, pipe) in line.chars().enumerate() {
                let index = format!("{}_{}", row, col);
                if loop_path.contains(&index) {
                    cprint!("<green>{}</>", pipe);
                } else {
                    print!("{}", pipe);
                };
            }
            print!("\n");
        }
    } else {
        panic!("File not found");
    }
}
