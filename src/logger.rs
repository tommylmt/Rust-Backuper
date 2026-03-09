use colored::Colorize;

pub fn debug(string: &str) {
    println!("{} - {string}", "[DEBUG]".purple());
}

pub fn error(string: &str) {
    println!("{} - {string}", "[ERROR]".red());
}

pub fn ok(string: &str) {
    println!("{} - {string}", "[OK]".blue());
}

pub fn critical(string: &str) {
    panic!("{} - {string}", "[CRITICAL]".red());
}

pub fn warning(string: &str) {
    println!("{} - {string}", "[WARNING]".yellow());
}

pub fn info(string: &str) {
    println!("{} - {string}", "[INFO]".green());
}