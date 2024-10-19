use log;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::str::Chars;

#[derive(Debug, Default, PartialEq, Eq, Hash)]
struct Point {
    row: u64,
    col: u64,
}
#[derive(Debug, Default, PartialEq, Eq, Hash)]
struct Number {
    start: Point,
    end: Point,
}
impl Number {
    /***
     * Is the given i,j coordinate inside this number?
     */
    fn is_inside(&self, i: u64, j: u64) -> bool {
        if i == self.start.row {
            if j >= self.start.col && j < self.end.col {
                return true;
            }
        }
        false
    }
    fn get_actual_number(&self, schematic: &Schematic) -> u64 {
        let row = &schematic[self.start.row as usize];
        let chars = &row[self.start.col as usize..self.end.col as usize];
        let s = String::from_iter(chars);
        // println!("{:?}", self);
        // println!("== {}", s);
        s.parse::<u64>().expect("number wasn't a number??")
    }
    fn _is_digit_part(&self, schematic: &Schematic, i: u64, j: u64) -> bool {
        fn check_position(schematic: &Schematic, i: u64, j: u64) -> bool {
            let c = schematic[i as usize][j as usize];
            if c.is_numeric() {
                return false;
            } else {
                if c == '.' {
                    return false;
                }
            }
            true
        }
        // Search around the digit in all directions
        // ...\|/...
        // ...-d-...
        // .../|\...
        //
        // looking for "symbols" (anything that is not a number and not a .)

        // i is row
        // j is col
        //
        let max_i = (schematic.len() as u64) - 1;
        let max_j = (schematic[0].len() as u64) - 1;
        // check up left
        if i > 0 && j > 0 {
            // if not, skip this check
            let i = i - 1;
            let j = j - 1;
            if check_position(&schematic, i, j) {
                return true;
            }
        }
        // check up straight
        if i > 0 {
            let i = i - 1;
            if check_position(&schematic, i, j) {
                return true;
            }
        }
        // check up right
        if i > 0 && j < max_j {
            // if not, skip this check
            let i = i - 1;
            let j = j + 1;
            if check_position(&schematic, i, j) {
                return true;
            }
        }
        // check left
        if j > 0 {
            let j = j - 1;
            if check_position(&schematic, i, j) {
                return true;
            }
        }
        // check right
        if j < max_j {
            let j = j + 1;
            if check_position(&schematic, i, j) {
                return true;
            }
        }
        // check down left
        if i < max_i && j > 0 {
            // if not, skip this check
            let i = i + 1;
            let j = j - 1;
            if check_position(&schematic, i, j) {
                return true;
            }
        }
        // check down straight
        if i < max_i {
            let i = i + 1;
            if check_position(&schematic, i, j) {
                return true;
            }
        }
        // check down right
        if i < max_i && j < max_j {
            // if not, skip this check
            let i = i + 1;
            let j = j + 1;
            if check_position(&schematic, i, j) {
                return true;
            }
        }

        false
    }
    fn is_part_number(&self, schematic: &Schematic) -> bool {
        for i in self.start.col..self.end.col {
            if self._is_digit_part(&schematic, self.start.row, i) {
                return true;
            }
        }
        false
    }
}
pub type Schematic = Vec<Vec<char>>;

/***
 * Find the numbers in the schematic
 */
fn find_the_numbers(schematic: &Schematic) -> Vec<Number> {
    let mut previous_was_number = false;
    let mut current_number = Number::default();
    let mut numbers: Vec<Number> = Vec::new();

    for (i, row) in schematic.into_iter().enumerate() {
        for (j, c) in row.into_iter().enumerate() {
            if c.is_numeric() {
                if !previous_was_number {
                    // println!("number begins");
                    current_number.start.row = i as u64;
                    current_number.start.col = j as u64;
                }
                // println!("{}", c);
                previous_was_number = true;
            } else {
                // the character is NOT numeric
                if previous_was_number {
                    // println!("number ends");
                    current_number.end.row = i as u64;
                    current_number.end.col = j as u64;
                    // println!("{:?}", current_number);
                    numbers.push(current_number);
                    current_number = Number::default();
                }
                previous_was_number = false;
            }
        }
        // at the end of the row, if we were still reading a number,
        // need to end it here!
        if previous_was_number {
            current_number.end.row = i as u64;
            current_number.end.col = row.len() as u64;
            // println!("{:?}", current_number);
            numbers.push(current_number);
            current_number = Number::default();
            previous_was_number = false;
        }
    }
    // println!("{:#?}", numbers);
    numbers
}

fn main() {
    env_logger::init();
    // let filename = "input/day3-test.txt";
    let filename = "input/day3.txt";
    log::info!("Using filename={filename}");
    let mut schematic: Schematic = Vec::new();
    let binding = read_to_string(filename).unwrap();
    for line in binding.lines() {
        let tvec: Vec<char> = line.chars().collect();
        schematic.push(tvec);
    }
    log::debug!("{:?}", schematic);

    // find all the numbers
    let numbers = find_the_numbers(&schematic);
    let mut part_number_total = 0;
    let mut part_numbers: Vec<&Number> = Vec::new();
    for number in &numbers {
        // println!("{:?}", number);
        let actual_num = number.get_actual_number(&schematic);
        if number.is_part_number(&schematic) {
            part_numbers.push(number);
            part_number_total += actual_num;
        }
        // println!("{}", number.is_part_number(&schematic));
    }

    println!("Part 1 total: {}", part_number_total);
    // println!("{:#?}", part_numbers);

    // TODO: scan the schematic for * symbols
    // for each found, calculate the i,j coordinates of each
    // position adjacent to it,
    // then for each of those coordinates:
    //      for each of the part_numbers:
    //          test if the coordinate is within it's range
    //          if it is, add a count for that symbol
    //
    //          if the symbol ends up with a score of 2, it counts

    find_stars(&schematic, &part_numbers);
}

fn square(i: usize, j: usize, max_i: usize, max_j: usize) -> Vec<(u64, u64)> {
    let i = i as i128;
    let j = j as i128;
    let max_i = max_i as i128;
    let max_j = max_j as i128;

    let square = vec![
        (i - 1, j - 1), // up left      \
        (i - 1, j),     // up           |
        (i - 1, j + 1), // up right     /
        (i, j + 1),     // right        -
        (i + 1, j + 1), // down right   \
        (i + 1, j),     // down         |
        (i + 1, j - 1), // down left    /
        (i, j - 1),     // left        -
    ];
    // convert to u64s here
    let result: Vec<_> = square
        .iter()
        .filter(|(x, y)| {
            // drop any that are out of range
            *x >= 0 && *y >= 0 && *x <= max_i && *y <= max_j
        })
        .map(|(x, y)| {
            // convert to u64
            (*x as u64, *y as u64)
        })
        .collect();
    result
}

fn find_stars(schematic: &Schematic, part_numbers: &Vec<&Number>) {
    let mut gear_ratio_total = 0;
    for (i, row) in schematic.into_iter().enumerate() {
        for (j, c) in row.into_iter().enumerate() {
            if *c == '*' {
                //
                // get coordinates around the given point. Requires the
                // max values to make sure it doens't return anything invalid
                //
                let s = square(i, j, schematic.len() - 1, row.len() - 1);
                let mut gear_numbers = HashSet::new();

                for (x, y) in s {
                    for part_number in part_numbers {
                        if part_number.is_inside(x, y) {
                            gear_numbers.insert(part_number);
                        }
                    }
                }
                if gear_numbers.len() == 2 {
                    // get the gear ratio
                    let gear_ratio = gear_numbers
                        .iter()
                        .map(|x| x.get_actual_number(&schematic))
                        .collect::<Vec<_>>()
                        .iter()
                        .product::<u64>();
                    gear_ratio_total += gear_ratio;
                }
            }
        }
    }
    println!("{gear_ratio_total}");
}
