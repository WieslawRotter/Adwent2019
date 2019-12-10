extern crate queues;
use queues::*;

struct Ampli {
    puzzle: Vec<i32>,
    index: usize,
    input: Queue<i32>,
    output: i32,
}

impl Ampli {
    pub fn new() -> Ampli {
        Ampli {
            puzzle: Vec::new(),
            index: 0,
            input: queue![],
            output: 0,
        }
    }

    fn set_puzzle(&mut self, puzzle: &Vec<i32>) {
        self.puzzle.clone_from(puzzle);
    }

    fn add_input(&mut self, input: i32) {
        self.input.add(input);
    }

    fn get_output(&self) -> i32 {
        self.output
    }

    fn run_comp(&mut self) -> bool {
        while self.puzzle[self.index] != 99 {
            let inst_1 = self.puzzle[self.index] % 100;
            let inst_2 = (self.puzzle[self.index] / 100) % 10;
            let inst_3 = (self.puzzle[self.index] / 1000) % 10;
            let inst_4 = self.puzzle[self.index] / 10000;
            let idx_1 = if inst_2 == 1 {
                self.index + 1
            } else {
                (self.puzzle[self.index + 1]) as usize
            };
            let idx_2 = if inst_3 == 1 {
                self.index + 2
            } else {
                (self.puzzle[self.index + 2]) as usize
            };
            let idx_3 = if inst_4 == 1 {
                self.index + 3
            } else {
                (self.puzzle[self.index + 3]) as usize
            };

            if inst_1 == 8 {
                self.puzzle[idx_3] = if self.puzzle[idx_1] == self.puzzle[idx_2] {
                    1
                } else {
                    0
                };
                self.index += 4;
            } else if inst_1 == 7 {
                self.puzzle[idx_3] = if self.puzzle[idx_1] < self.puzzle[idx_2] {
                    1
                } else {
                    0
                };
                self.index += 4;
            } else if inst_1 == 6 {
                if self.puzzle[idx_1] == 0 {
                    self.index = self.puzzle[idx_2] as usize;
                } else {
                    self.index += 3;
                }
            } else if inst_1 == 5 {
                if self.puzzle[idx_1] != 0 {
                    self.index = self.puzzle[idx_2] as usize;
                } else {
                    self.index += 3;
                }
            } else if inst_1 == 4 {
                self.output = self.puzzle[idx_1];
                self.index += 2;
            } else if inst_1 == 3 {
                if self.input.size() == 0 {
                    return true;
                }
                self.puzzle[idx_1] = self.input.peek().unwrap();
                self.input.remove();
                self.index += 2;
            } else if inst_1 == 2 {
                let new_value = self.puzzle[idx_1] * self.puzzle[idx_2];
                let new_index = self.puzzle[self.index + 3] as usize;
                self.puzzle[new_index] = new_value;
                self.index += 4;
            } else if inst_1 == 1 {
                let new_value = self.puzzle[idx_1] + self.puzzle[idx_2];
                let new_index = self.puzzle[self.index + 3] as usize;
                self.puzzle[new_index] = new_value;
                self.index += 4;
            }
        }
        return false;
    }
}

fn main() {
    let mut puzzle = vec![
        3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 34, 59, 68, 89, 102, 183, 264, 345, 426, 99999, 3,
        9, 102, 5, 9, 9, 1001, 9, 5, 9, 4, 9, 99, 3, 9, 101, 3, 9, 9, 1002, 9, 5, 9, 101, 5, 9, 9,
        1002, 9, 3, 9, 1001, 9, 5, 9, 4, 9, 99, 3, 9, 101, 5, 9, 9, 4, 9, 99, 3, 9, 102, 4, 9, 9,
        101, 3, 9, 9, 102, 5, 9, 9, 101, 4, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 5, 9, 1001, 9, 2, 9, 4,
        9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3,
        9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9,
        2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
        99, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9,
        101, 1, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9,
        4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 99,
        3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001,
        9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4,
        9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99, 3,
        9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9,
        2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
        3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9,
        1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9,
        9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3,
        9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 99,0,0,
    ];
    let mut max = 0;
    let mut vec = vec![5, 6, 7, 8, 9];
    let mut all_perm = Vec::new();

    perm(5, &mut vec, &mut all_perm);
    
    for permutattion in all_perm {
        let mut aaa1 = Ampli::new();
        let mut aaa2 = Ampli::new();
        let mut aaa3 = Ampli::new();
        let mut aaa4 = Ampli::new();
        let mut aaa5 = Ampli::new();
        aaa1.set_puzzle(&puzzle);
        aaa2.set_puzzle(&puzzle);
        aaa3.set_puzzle(&puzzle);
        aaa4.set_puzzle(&puzzle);
        aaa5.set_puzzle(&puzzle);
        aaa1.add_input(permutattion[0]);
        aaa1.run_comp();
        aaa2.add_input(permutattion[1]);
        aaa2.run_comp();
        aaa3.add_input(permutattion[2]);
        aaa3.run_comp();
        aaa4.add_input(permutattion[3]);
        aaa4.run_comp();
        aaa5.add_input(permutattion[4]);
        while aaa5.run_comp() {
            aaa1.add_input(aaa5.get_output());
            aaa1.run_comp();
            aaa2.add_input(aaa1.get_output());
            aaa2.run_comp();
            aaa3.add_input(aaa2.get_output());
            aaa3.run_comp();
            aaa4.add_input(aaa3.get_output());
            aaa4.run_comp();
            aaa5.add_input(aaa4.get_output());
        }

        if aaa5.get_output() > max {
            max = aaa5.get_output();
        }
    }

    println!("Part Two : {}", max);
}

fn perm(k: usize, vector: &mut Vec<i32>, all_perm: &mut Vec<Vec<i32>>) {
    if k == 0 {
        all_perm.push(vector.to_vec());
    } else {
        for idx in 0..k {
            vector.swap(idx, k - 1);
            perm(k - 1, vector, all_perm);
            vector.swap(idx, k - 1);
        }
    }
}
