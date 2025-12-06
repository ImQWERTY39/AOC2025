#![allow(unused)]

use std::collections::HashSet;

pub fn day5() {
    let file_content = std::fs::read_to_string("./inputs/question5.txt").unwrap();
    let mut count = 0;

    let mut data = file_content.split("\n\n");
    let ranges_data = data.next().unwrap();
    let id_data = data.next().unwrap();

    let ranges = ranges_data
        .lines()
        .map(|x| {
            let mut bounds = x.split("-");
            let lower_bound = bounds.next().unwrap().parse::<u128>().unwrap();
            let upper_bound = bounds.next().unwrap().parse::<u128>().unwrap();
            (lower_bound, upper_bound)
        })
        .collect::<Vec<_>>();

    'id_loop: for id in id_data.lines().map(|x| x.parse::<u128>().unwrap()) {
        for (l, u) in ranges.iter() {
            if *l <= id && id <= *u {
                count += 1;
                continue 'id_loop;
            }
        }
    }

    println!("Answer: {}", count);
}

pub fn day5_part2() {
    let file_content = std::fs::read_to_string("./inputs/check.txt").unwrap();
    let mut count = 0;

    let ranges = file_content
        .lines()
        .map(|x| {
            let mut bounds = x.split("-");
            let lower_bound = bounds.next().unwrap().parse::<u128>().unwrap();
            let upper_bound = bounds.next().unwrap().parse::<u128>().unwrap();
            (lower_bound, upper_bound)
        })
        .collect::<Vec<_>>();

    let mut prev = ranges[0];

    for &(start, end) in &ranges[1..] {
        if start <= prev.1 {
            prev.1 = prev.1.max(end);
        } else {
            count += prev.1 - prev.0 + 1;
            prev = (start, end);
        }
    }

    count += prev.1 - prev.0 + 1;
    println!("Answer: {}", count);
}
