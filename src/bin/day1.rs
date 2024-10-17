use std::fs::read_to_string;

fn get_calibration_value(line: &str) -> u64 {
    let nums: Vec<char> = line
        .chars()
        .filter(|x| x.is_numeric())
        .collect();
    let first_and_last = vec![nums.first().unwrap(), nums.last().unwrap()];

    (String::from_iter(first_and_last)).parse::<u64>().unwrap()
}

fn part1() {
    println!("Part 1");

    let mut total = 0;
    for line in read_to_string("input/day1.txt").unwrap().lines() {
        total += get_calibration_value(line);
    }

    println!("The calibration value for this input file is: {}", total);
    
}

#[derive(Debug)]
enum Number {
    Digit(u8),
    Word(String)
}

impl Number {
    fn to_int(&self) -> u8 {
        match self {
            Number::Digit(val) => *val,
            Number::Word(val) => match val.as_str() {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                "ten" => 10,
                _ => 0
            },
        }
    }
}

fn to_two_digit_int(a: &Number, b: &Number) -> u16 {
    let a_str: String = a.to_int().to_string();
    let b_str: String = b.to_int().to_string();
    let res = a_str + &b_str;

    res.parse().unwrap()
}

fn part2() {
    println!("Part 2");
    // let filename = "input/day1-2-test.txt";
    let filename = "input/day1.txt";

    let words = vec![
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten"
    ];

    let mut all_numbers: Vec<u16> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        // println!("---------------------------");
        // println!("{}", line);
        // println!("---------------------------");
        let mut numbers: Vec<(Number, usize)> = Vec::new();
        for word in &words {
            // find all the word numbers and their positions
            line.match_indices(word).for_each(|(i, k)| {
                let a = (Number::Word(word.to_string()), i);
                // println!("{:?}", a);
                numbers.push(a);
            });
        }
        // find the numeric digits and their positions
        line.char_indices().for_each(|(i, c)| {
            if c.is_numeric() {
                let a = (Number::Digit(c.to_string().parse::<u8>().unwrap()), i);
                // println!("{:?}", a);
                numbers.push(a);
            }
        });

        numbers.sort_by_key(|k| k.1);
        // println!("{:?}", numbers);
        let first = &numbers
            .first()
            .unwrap()
            .0;
        let last = &numbers
            .last()
            .unwrap()
            .0;
        // println!("{:?}//{:?}", first, last);
        // println!("{}", to_two_digit_int(first, last));
        all_numbers.push(to_two_digit_int(first, last));
    }


    // let mut tosort = vec![("five", 5), ("six", 4)];
    // tosort.sort_by_key(|k| k.1);

    // println!("{:?}", all_numbers);
    println!("The updated calibration value is: {}",
             all_numbers.iter().sum::<u16>());

}

fn main() {
    part1();
    part2();
}
