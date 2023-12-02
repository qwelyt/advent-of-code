use std::fmt::Display;
use std::time::Instant;

pub fn time<T: Display>(f: fn(&str) -> T, input: &str, part: &str) {
    let start = Instant::now();
    let result = f(input);
    let end = Instant::now();
    println!("Part {}: {}, took {}ns", part, result, end.duration_since(start).as_nanos())
}