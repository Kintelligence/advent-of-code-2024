use shared::Solution;

pub fn day(index: u32) -> &'static (fn(&str) -> Solution, &'static str, &'static str) {
    match index {
        1 => &(day_01::part_1, day_01::_INPUT, "01.1"),
        2 => &(day_01::part_2, day_01::_INPUT, "01.2"),
        3 => &(day_02::part_1, day_02::_INPUT, "02.1"),
        4 => &(day_02::part_2, day_02::_INPUT, "02.2"),
        5 => &(day_03::part_1, day_03::_INPUT, "03.1"),
        6 => &(day_03::part_2, day_03::_INPUT, "03.2"),
        7 => &(day_04::part_1, day_04::_INPUT, "04.1"),
        8 => &(day_04::part_2, day_04::_INPUT, "04.2"),
        9 => &(day_05::part_1, day_05::_INPUT, "05.1"),
        10 => &(day_05::part_2, day_05::_INPUT, "05.2"),
        11 => &(day_06::part_1, day_06::_INPUT, "06.1"),
        12 => &(day_06::part_2, day_06::_INPUT, "06.2"),
        13 => &(day_07::part_1, day_07::_INPUT, "07.1"),
        14 => &(day_07::part_2, day_07::_INPUT, "07.2"),
        15 => &(day_08::part_1, day_08::_INPUT, "08.1"),
        16 => &(day_08::part_2, day_08::_INPUT, "08.2"),
        17 => &(day_09::part_1, day_09::_INPUT, "09.1"),
        18 => &(day_09::part_2, day_09::_INPUT, "09.2"),
        19 => &(day_10::part_1, day_10::_INPUT, "10.1"),
        20 => &(day_10::part_2, day_10::_INPUT, "10.2"),
        21 => &(day_11::part_1, day_11::_INPUT, "11.1"),
        22 => &(day_11::part_2, day_11::_INPUT, "11.2"),
        23 => &(day_12::part_1, day_12::_INPUT, "12.1"),
        24 => &(day_12::part_2, day_12::_INPUT, "12.2"),
        25 => &(day_13::part_1, day_13::_INPUT, "13.1"),
        26 => &(day_13::part_2, day_13::_INPUT, "13.2"),
        27 => &(day_14::part_1, day_14::_INPUT, "14.1"),
        28 => &(day_14::part_2, day_14::_INPUT, "14.2"),
        29 => &(day_15::part_1, day_15::_INPUT, "15.1"),
        30 => &(day_15::part_2, day_15::_INPUT, "15.2"),
        31 => &(day_16::part_1, day_16::_INPUT, "16.1"),
        32 => &(day_16::part_2, day_16::_INPUT, "16.2"),
        33 => &(day_17::part_1, day_17::_INPUT, "17.1"),
        34 => &(day_17::part_2, day_17::_INPUT, "17.2"),
        35 => &(day_18::part_1, day_18::_INPUT, "18.1"),
        36 => &(day_18::part_2, day_18::_INPUT, "18.2"),
        37 => &(day_19::part_1, day_19::_INPUT, "19.1"),
        38 => &(day_19::part_2, day_19::_INPUT, "19.2"),
        39 => &(day_20::part_1, day_20::_INPUT, "20.1"),
        40 => &(day_20::part_2, day_20::_INPUT, "20.2"),
        41 => &(day_21::part_1, day_21::_INPUT, "21.1"),
        42 => &(day_21::part_2, day_21::_INPUT, "21.2"),
        43 => &(day_22::part_1, day_22::_INPUT, "22.1"),
        44 => &(day_22::part_2, day_22::_INPUT, "22.2"),
        45 => &(day_23::part_1, day_23::_INPUT, "23.1"),
        46 => &(day_23::part_2, day_23::_INPUT, "23.2"),
        47 => &(day_24::part_1, day_24::_INPUT, "24.1"),
        48 => &(day_24::part_2, day_24::_INPUT, "24.2"),
        49 => &(day_25::part_1, day_25::_INPUT, "25.1"),
        _ => panic!("Index out of bounds"),
    }
}
