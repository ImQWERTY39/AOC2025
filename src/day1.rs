#![allow(unused)]

pub fn day1() {
    let file_content = std::fs::read_to_string("./inputs/question1.txt").unwrap();

    let mut dial = 50;
    let mut count = 0;

    for i in file_content.lines() {
        let mut chars = i.chars();
        let direction = chars.next().unwrap();
        let times: i32 = chars.collect::<String>().parse().unwrap();

        if direction == 'R' {
            dial = (dial + times) % 100;
        } else if direction == 'L' {
            dial = (dial - times + 100) % 100;
        }

        if dial == 0 {
            count += 1;
        }
    }

    println!("Answer: {}", count);
}

pub fn day1_part2() {
    let file_content = std::fs::read_to_string("./inputs/question1.txt").unwrap();

    let mut dial = 50;
    let mut count = 0;

    for i in file_content.lines() {
        let mut chars = i.chars();
        let direction = chars.next().unwrap();
        let times: i32 = chars.collect::<String>().parse().unwrap();

        for _ in 0..times {
            if direction == 'R' {
                dial = (dial + 1) % 100;
            } else if direction == 'L' {
                dial = (dial + 99) % 100;
            }

            if dial == 0 {
                count += 1;
            }
        }
    }

    println!("Answer: {}", count);
}
