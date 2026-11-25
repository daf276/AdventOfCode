use std::{
    fmt::Display,
    fs,
    str::FromStr,
    time::{Duration, Instant},
};

pub fn read_input(day: usize, variant: Option<&str>) -> String {
    let suffix = variant.map(|v| format!(".{v}")).unwrap_or_default();
    let path = format!("{}/inputs/day{day:02}{suffix}", env!("CARGO_MANIFEST_DIR"));
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"))
}

pub fn timed<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let start = Instant::now();
    let result = f();
    (result, start.elapsed())
}

pub fn parse_nums<T: FromStr + Display>(l: &str) -> Vec<T> {
    l.lines().map(parse_num).collect()
}

pub fn parse_nums_separator<T: FromStr + Display>(l: &str, separator: char) -> Vec<T> {
    l.trim().split(separator).map(parse_num).collect()
}

pub fn parse_num<T: FromStr + Display>(s: &str) -> T {
    s.parse().unwrap_or_else(|_| panic!("Invalid number {s}"))
}
