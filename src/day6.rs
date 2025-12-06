#![allow(unused)]

pub fn day6() {
    let file_content = std::fs::read_to_string("./inputs/question6.txt").unwrap();
    let mut sum = 0;

    let lines = file_content.lines();
    let collected: Vec<&str> = lines.collect();
    let (num_lines, last) = collected.split_at(collected.len() - 1);

    let numbers: Vec<Vec<u128>> = num_lines
        .into_iter()
        .map(|x| {
            x.split_whitespace()
                .map(|num| match num.parse::<u128>() {
                    Ok(i) => i,
                    Err(_) => panic!("{}", num),
                })
                .collect::<Vec<u128>>()
        })
        .collect();
    let operations: Vec<&str> = last[0].split_whitespace().collect();

    for j in 0..numbers[0].len() {
        let col = (0..numbers.len()).map(|i| numbers[i][j]);

        sum += if operations[j] == "+" {
            col.sum::<u128>()
        } else {
            col.product::<u128>()
        }
    }

    println!("Answer: {}", sum);
}

pub fn day6_part2() {
    let file_content = std::fs::read_to_string("./inputs/question6.txt").unwrap();

    let lines = file_content.lines();
    let collected: Vec<&str> = lines.collect();
    let (num_lines, last) = collected.split_at(collected.len() - 1);
    let (operations, number_of_spaces) = parse_operations(last[0]);

    let numbers = parse_nums(num_lines, number_of_spaces);
    println!("Answer: {}", compute(numbers, operations));
}

fn parse_operations(line: &str) -> (Vec<char>, Vec<usize>) {
    let mut operations = Vec::new();
    let mut number_of_spaces = Vec::new();

    let mut num_spaces = 0;
    let mut first = true;
    for i in line.chars() {
        if i == ' ' {
            num_spaces += 1;
            continue;
        }

        operations.push(i);
        if first {
            first = false;
            continue;
        }
        number_of_spaces.push(num_spaces);
        num_spaces = 0;
    }

    if num_spaces != 0 {
        number_of_spaces.push(num_spaces + 1);
    }

    (operations, number_of_spaces)
}

fn parse_nums(num_lines: &[&str], number_of_spaces: Vec<usize>) -> Vec<Vec<Vec<u8>>> {
    let mut nums = Vec::new();

    for line in num_lines {
        let mut num_digit_index = 0;
        let mut line_vec = Vec::new();
        let mut num_builder = Vec::new();

        let mut iter = line.chars();
        while num_digit_index < number_of_spaces.len() {
            for _ in 0..number_of_spaces[num_digit_index] {
                let c = iter.next().unwrap();
                if c == ' ' {
                    num_builder.push(0);
                } else {
                    num_builder.push(c as u8 - 48);
                }
            }

            line_vec.push(num_builder);
            num_builder = Vec::new();
            num_digit_index += 1;
            iter.next();
        }

        nums.push(line_vec);
        line_vec = Vec::new();
    }

    nums
}

fn compute(nums: Vec<Vec<Vec<u8>>>, operations: Vec<char>) -> u128 {
    let mut sum = 0;

    for j in 0..nums[0].len() {
        let unarranged_digits = (0..nums.len()).map(|i| &nums[i][j]).collect::<Vec<_>>();

        let mut actual_nums = Vec::new();
        for a in 0..unarranged_digits[0].len() {
            let mut num: u128 = 0;

            for b in 0..unarranged_digits.len() {
                num = num * 10 + unarranged_digits[b][a] as u128;
            }

            while num % 10 == 0 {
                num /= 10;
            }

            actual_nums.push(num);
        }

        sum += if operations[j] == '+' {
            actual_nums.iter().sum::<u128>()
        } else {
            actual_nums.iter().product::<u128>()
        };
    }

    sum
}
