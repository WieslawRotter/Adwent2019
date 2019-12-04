struct DigitIter(usize, usize);

impl Iterator for DigitIter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let ret = self.0 % self.1;
            self.0 /= self.1;
            Some(ret)
        }
    }
}

fn main() {
    let mut count_part1 = 0;
    let mut count_part2 = 0;
    for number in 387638..919123 {
        let mut prew = 10;
        let mut is_ok_part1: bool = false;
        let mut is_ok_part2: bool = false;
        let mut last_equal = 10;
        let mut finded = 10;
        for digit in DigitIter(number, 10) {
            if prew == 10 {
                prew = digit;
            } else {
                if finded != digit && finded != 10 {
                    is_ok_part2 = true;
                    break;
                }
                if prew == digit {
                    if last_equal == digit {
                        finded = 10;
                        is_ok_part2 = false;
                    } else {
                        last_equal = digit;
                        finded = digit;
                        is_ok_part2 = true;
                    }
                }
                prew = digit;
            }
        }

        let mut prew = 10;
        for digit in DigitIter(number, 10) {
            if prew == digit {
                is_ok_part1 = true;
                break;
            }

            prew = digit;
        }
        let mut prew = 10;
        for digit in DigitIter(number, 10) {
            if prew == 10 {
                prew = digit;
            } else {
                if prew < digit {
                    is_ok_part1 = false;
                    break;
                }
                prew = digit;
            }
        }

        if is_ok_part1 {
            count_part1 += 1;
        }

        if is_ok_part2 && is_ok_part1 {
            count_part2 += 1;
        }
    }
    println!("Part One: {},Part Two: {}", count_part1, count_part2);
}
