#[allow(unused)]
pub fn day4() {
    let file_content = std::fs::read_to_string("./inputs/question4.txt").unwrap();
    let mut count = 0;

    let matrix = file_content
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    for (i, row) in matrix.iter().enumerate() {
        for (j, elem) in row.iter().enumerate() {
            if *elem == '@' && count_neighbour(&matrix, i, j) < 4 {
                count += 1;
            }
        }
    }

    println!("Answer: {}", count);
}

fn count_neighbour(matrix: &[Vec<char>], i: usize, j: usize) -> u8 {
    let mut count = 0;

    count += get(matrix, i as isize - 1, j as isize - 1).is_some_and(|x| *x == '@') as u8;
    count += get(matrix, i as isize - 1, j as isize).is_some_and(|x| *x == '@') as u8;
    count += get(matrix, i as isize - 1, j as isize + 1).is_some_and(|x| *x == '@') as u8;
    count += get(matrix, i as isize, j as isize - 1).is_some_and(|x| *x == '@') as u8;
    count += get(matrix, i as isize, j as isize + 1).is_some_and(|x| *x == '@') as u8;
    count += get(matrix, i as isize + 1, j as isize - 1).is_some_and(|x| *x == '@') as u8;
    count += get(matrix, i as isize + 1, j as isize).is_some_and(|x| *x == '@') as u8;
    count += get(matrix, i as isize + 1, j as isize + 1).is_some_and(|x| *x == '@') as u8;

    count
}

#[allow(unused)]
fn get(matrix: &[Vec<char>], i: isize, j: isize) -> Option<&char> {
    if i < 0 || j < 0 {
        None
    } else {
        matrix.get(i as usize).map(|x| x.get(j as usize))?
    }
}

#[allow(unused)]
pub fn day4_part2() {
    let file_content = std::fs::read_to_string("./inputs/question4.txt").unwrap();
    let mut count = 0;
    let mut matrix = file_content
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut change = Vec::new();

    loop {
        change.clear();

        for (i, row) in matrix.iter().enumerate() {
            for (j, elem) in row.iter().enumerate() {
                if *elem == '@' && count_neighbour(&matrix, i, j) < 4 {
                    count += 1;
                    change.push((i, j));
                }
            }
        }

        if change.is_empty() {
            break;
        }

        change.iter().for_each(|(i, j)| matrix[*i][*j] = '.');
    }

    println!("Answer: {}", count);
}
