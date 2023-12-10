static ZERO_VALUE: u32 = '0' as u32;

pub fn solve_part_one(data: &str) -> u32 {
    data.lines()
        .map(|line| {
            let (_, first, last) = line
                .chars()
                .filter_map(|char| {
                    if char.is_ascii_digit() {
                        Some(char as u32 - ZERO_VALUE)
                    } else {
                        None
                    }
                })
                .fold((false, 0, 0), |(first_done, first, _), value| {
                    if first_done {
                        (true, first, value)
                    } else {
                        (true, value, value)
                    }
                });
            first * 10 + last
        })
        .sum()
}

pub fn solve_part_two(data: &str) -> u32 {
    data.lines()
        .map(|line| {
            let (_, first, last) = line
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
                .chars()
                .filter_map(|char| {
                    if char.is_ascii_digit() {
                        Some(char as u32 - ZERO_VALUE)
                    } else {
                        None
                    }
                })
                .fold((false, 0, 0), |(first_done, first, _), value| {
                    if first_done {
                        (true, first, value)
                    } else {
                        (true, value, value)
                    }
                });
            first * 10 + last
        })
        .sum()
}
