use std::time::Duration;

use runner::day;
use shared::{parse::Parsable, *};
use std::panic;

extern crate shared;

fn main() {
    let mut time: Duration = Duration::new(0, 0);
    for i in 1..=49 {
        let (function, input, id) = day(i);
        let day = id.bytes().next_number().unwrap();

        let result = panic::catch_unwind(|| execute(function, input, id, day_name(day)));

        match result {
            Ok(duration) => time += duration,
            Err(_) => {
                println!("Day {} failed", day);
            }
        }
    }
    total(time);
}
