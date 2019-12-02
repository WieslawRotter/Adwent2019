fn main() {
    let mut puzzle = [
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 13, 1, 19, 1, 6, 19, 23, 2, 6, 23, 27,
        1, 5, 27, 31, 2, 31, 9, 35, 1, 35, 5, 39, 1, 39, 5, 43, 1, 43, 10, 47, 2, 6, 47, 51, 1, 51,
        5, 55, 2, 55, 6, 59, 1, 5, 59, 63, 2, 63, 6, 67, 1, 5, 67, 71, 1, 71, 6, 75, 2, 75, 10, 79,
        1, 79, 5, 83, 2, 83, 6, 87, 1, 87, 5, 91, 2, 9, 91, 95, 1, 95, 6, 99, 2, 9, 99, 103, 2, 9,
        103, 107, 1, 5, 107, 111, 1, 111, 5, 115, 1, 115, 13, 119, 1, 13, 119, 123, 2, 6, 123, 127,
        1, 5, 127, 131, 1, 9, 131, 135, 1, 135, 9, 139, 2, 139, 6, 143, 1, 143, 5, 147, 2, 147, 6,
        151, 1, 5, 151, 155, 2, 6, 155, 159, 1, 159, 2, 163, 1, 9, 163, 0, 99, 2, 0, 14, 0,
    ];
    println!("Part One : {}", part_one(&mut puzzle));
    println!("Part Two: {}", part_two(&mut puzzle));
}

fn part_one(puzzle: &mut [usize]) -> usize {
    let mut index = 0;
    let mut puzzle_copy = vec![0; 173];
    puzzle_copy[..173].clone_from_slice(&puzzle);
    while puzzle_copy[index] != 99 {
        let new_vlue = if puzzle_copy[index] == 1 {
            puzzle_copy[puzzle_copy[index + 1]] + puzzle_copy[puzzle_copy[index + 2]]
        } else {
            puzzle_copy[puzzle_copy[index + 1]] * puzzle_copy[puzzle_copy[index + 2]]
        };
        let new_index = puzzle_copy[index + 3];
        puzzle_copy[new_index] = new_vlue;
        index += 4;
    }
    return puzzle_copy[0];
}

fn part_two(puzzle: &mut [usize]) -> usize {
    let mut noun = 0;
    let mut verb = 0;
    while part_one(puzzle) != 19690720 {
        if noun == 99 {
            noun = 0;
            verb += 1;
        } else {
            noun += 1;
        }
        puzzle[1] = noun;
        puzzle[2] = verb;
    }
    return 100 * puzzle[1] + puzzle[2];
}
