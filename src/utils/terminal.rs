pub fn green(value: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", value)
}

pub fn yellow(value: &str) -> String {
    format!("\x1b[33m{}\x1b[0m", value)
}

pub fn red(value: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", value)
}

pub fn blue(value: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", value)
}

pub fn purple(value: &str) -> String {
    format!("\x1b[35m{}\x1b[0m", value)
}
