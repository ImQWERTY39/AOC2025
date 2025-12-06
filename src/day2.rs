#![allow(unused)]

pub fn day2() {
    let file_content = std::fs::read_to_string("./inputs/question2.txt").unwrap();
    let id_ranges = file_content.trim().split(",").map(|x| {
        let mut range_bounds = x.split("-");
        let lower_bound: u128 = range_bounds.next().unwrap().parse().unwrap();
        let upper_bound: u128 = range_bounds.next().unwrap().parse().unwrap();

        (lower_bound, upper_bound)
    });

    let mut id_sum: u128 = 0;
    for (l, u) in id_ranges {
        for id in l..=u {
            let id_string = id.to_string();

            let mid = id_string.len() / 2;
            if id_string.len() % 2 == 0 && id_string[..mid] == id_string[mid..] {
                id_sum += id;
            }
        }
    }

    println!("{}", id_sum);
}

pub fn day2_part2() {
    let file_content = std::fs::read_to_string("./inputs/question2.txt").unwrap();
    let id_ranges = file_content.trim().split(",").map(|x| {
        let mut range_bounds = x.split("-");
        let lower_bound: u128 = range_bounds.next().unwrap().parse().unwrap();
        let upper_bound: u128 = range_bounds.next().unwrap().parse().unwrap();

        (lower_bound, upper_bound)
    });

    let mut id_sum: u128 = 0;
    for (l, u) in id_ranges {
        for id in l..=u {
            let id_string = id.to_string();
            if check_repeating_in_string(&id_string) {
                id_sum += id;
            }
        }
    }

    println!("{}", id_sum);
}

fn check_repeating_in_string(string: &str) -> bool {
    let len = string.len();
    for i in 1..=(len / 2) {
        if len % i != 0 {
            continue;
        }

        let sub_str_rep = string[..i].repeat(len / i as usize);

        if sub_str_rep == string {
            return true;
        }
    }

    false
}
