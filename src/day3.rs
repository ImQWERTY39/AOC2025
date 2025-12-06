#![allow(unused)]

pub fn day3() {
    let file_content = std::fs::read_to_string("./inputs/question3.txt").unwrap();

    let mut sum: u32 = 0;
    for joltage in file_content.lines() {
        let mut tens_digit = 0;
        let mut ones_digit = 0;

        for chars in joltage.as_bytes().windows(2) {
            let digit = chars[0] - ('0' as u8);
            let next_digit = chars[1] - ('0' as u8);

            if digit > tens_digit {
                tens_digit = digit;
                ones_digit = next_digit;

                continue;
            }

            ones_digit = ones_digit.max(digit).max(next_digit);
        }

        sum += (tens_digit * 10 + ones_digit) as u32;
    }

    println!("Answer: {}", sum);
}

pub fn day3_part2() {
    let file_content = std::fs::read_to_string("./inputs/question3.txt").unwrap();
    let mut sum = 0u128;

    for joltage in file_content.lines() {
        let mut j = joltage.to_string();

        while j.len() > 12 {
            let mut l = Vec::new();

            for i in 0..j.len() {
                let mut candidate = j[..i].to_string();
                candidate.push_str(&j[i + 1..]);

                l.push(candidate);
            }

            j = l.iter().max().unwrap().to_string();
        }

        sum += j.parse::<u128>().unwrap();
    }

    println!("Answer: {}", sum);
}
