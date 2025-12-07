#![allow(unused)]

use std::collections::HashMap;

pub fn day7() {
    let file_content = std::fs::read_to_string("./inputs/question7.txt").unwrap();
    let mut matrix: Vec<Vec<char>> = file_content.lines().map(|x| x.chars().collect()).collect();
    let mut count = 0;

    for i in 1..matrix.len() {
        for j in 0..matrix[i].len() {
            if (matrix[i][j] == '.') && (matrix[i - 1][j] == '|' || matrix[i - 1][j] == 'S') {
                matrix[i][j] = '|';
            } else if matrix[i][j] == '^' && matrix[i - 1][j] == '|' {
                matrix[i][j - 1] = '|';
                matrix[i][j + 1] = '|';
                count += 1;
            }
        }
    }

    println!("Answer: {}", count);
}

pub fn day7_part2() {
    let file_content = std::fs::read_to_string("./inputs/question7.txt").unwrap();
    let mut matrix: Vec<Vec<char>> = file_content.lines().map(|x| x.chars().collect()).collect();
    let count = solve_grid(&mut matrix);

    println!("Answer: {}", count);
}

fn solve_grid(matrix: &mut [Vec<char>]) -> usize {
    let index = matrix[0]
        .iter()
        .enumerate()
        .find(|&(_, &c)| c == 'S')
        .unwrap()
        .0;

    let mut map = HashMap::new();
    solve_grid_helper(matrix, 1, index, &mut map)
}

fn solve_grid_helper(
    matrix: &mut [Vec<char>],
    i: usize,
    j: usize,
    map: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if matrix[i][j] == '|' {
        return map[&(i, j)];
    }

    matrix[i][j] = '|';

    if i + 1 == matrix.len() {
        return 1;
    }

    if matrix[i + 1][j] == '^' {
        let left_path = solve_grid_helper(matrix, i + 1, j - 1, map);
        let right_path = solve_grid_helper(matrix, i + 1, j + 1, map);

        map.insert((i + 1, j - 1), left_path);
        map.insert((i + 1, j + 1), right_path);
        map.insert((i, j), left_path + right_path);

        return left_path + right_path;
    }

    let num = solve_grid_helper(matrix, i + 1, j, map);
    map.insert((i, j), num);

    return num;
}
