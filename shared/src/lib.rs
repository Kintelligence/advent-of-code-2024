use std::{
    fmt::{Display, Formatter},
    time::{Duration, Instant},
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Solution {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    Str(String),
    None,
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I8(x) => x.fmt(f),
            Self::I16(x) => x.fmt(f),
            Self::I32(x) => x.fmt(f),
            Self::I64(x) => x.fmt(f),
            Self::I128(x) => x.fmt(f),
            Self::Isize(x) => x.fmt(f),
            Self::U8(x) => x.fmt(f),
            Self::U16(x) => x.fmt(f),
            Self::U32(x) => x.fmt(f),
            Self::U64(x) => x.fmt(f),
            Self::U128(x) => x.fmt(f),
            Self::Usize(x) => x.fmt(f),
            Self::Str(x) => x.fmt(f),
            Self::None => panic!("Cannot format NOTHING!"),
        }
    }
}

macro_rules! impl_from {
    ($type_:ident, $kind_:ident) => {
        impl From<$type_> for Solution {
            fn from(sol: $type_) -> Self {
                Self::$kind_(sol)
            }
        }
    };
}

impl_from!(i8, I8);
impl_from!(i16, I16);
impl_from!(i32, I32);
impl_from!(i64, I64);
impl_from!(i128, I128);
impl_from!(isize, Isize);
impl_from!(u8, U8);
impl_from!(u16, U16);
impl_from!(u32, U32);
impl_from!(u64, U64);
impl_from!(u128, U128);
impl_from!(usize, Usize);
impl_from!(String, Str);

impl From<&str> for Solution {
    fn from(sol: &str) -> Self {
        Self::Str(sol.to_owned())
    }
}

impl<T> From<Option<T>> for Solution {
    fn from(_value: Option<T>) -> Self {
        Self::None
    }
}

use colored::Colorize;

pub fn execute(f: &dyn Fn(&str) -> Solution, input: &str, day: &str, name: &str) -> Duration {
    let start = Instant::now();
    let result = f(input);
    let time = start.elapsed();

    if let Solution::None = result {
        return Duration::ZERO;
    }

    let ratio = time.as_micros() as f64 / (Duration::from_secs(1) / 50).as_micros() as f64;

    let color = (ratio * 255.0).min(255.0) as u8;

    println!(
        "{: >12} {:} => {}",
        format!("{:#?}", time).truecolor(color, 255 - color, 0),
        format!("{}: {}", day, name).cyan().bold(),
        format!("{}", result).bold(),
    );

    time
}

pub fn total(time: Duration) {
    let ratio = time.as_micros() as f64 / (Duration::from_secs(1)).as_micros() as f64;

    let color = (ratio * 255.0).min(255.0) as u8;

    let remaining = Duration::from_secs(1) - time;

    println!(
        "{: >12} {}",
        format!("{:#?}", time).truecolor(color, 255 - color, 0),
        "Total".cyan().bold(),
    );
    println!(
        "{: >12} {}",
        format!("{:#?}", remaining).truecolor(color, 255 - color, 0),
        "Remaining".cyan().bold(),
    );
}

pub fn day_name(day: u32) -> &'static str {
    match day {
        1 => "Historian Hysteria",
        2 => "Red-Nosed Reports",
        3 => "Mull It Over",
        4 => "Ceres Search",
        5 => "Print Queue",
        6 => "Guard Gallivant",
        7 => "Bridge Repair",
        8 => "Resonant Collinearity",
        9 => "Disk Fragmenter",
        10 => "Hoof It",
        11 => "Plutonian Pebbles",
        12 => "Garden Groups",
        13 => "Claw Contraption",
        14 => "Restroom Redoubt",
        15 => "Warehouse Woes",
        16 => "Reindeer Maze",
        17 => "Chronospatial Computer",
        18 => "Unnamed",
        19 => "Unnamed",
        20 => "Unnamed",
        21 => "Unnamed",
        22 => "Unnamed",
        23 => "Unnamed",
        _ => "Unnamed",
    }
}

pub mod grid;
pub mod math;
pub mod misc;
pub mod parse;
pub mod points;
