use std::fs::read_to_string;

#[derive(Debug)]
struct GameMove {
    red: u16,
    green: u16,
    blue: u16,
}

impl GameMove {
    fn from_string(move_string: &str) -> Self {
        let mut result = Self {
            red: 0,
            green: 0,
            blue: 0
        };

        // println!("{}", move_string);
        for m in move_string.split(", ").collect::<Vec<_>>() {
            let trimmed_m = m.trim();
            // println!("-{}-", trimmed_m);
            let parts: Vec<_> = trimmed_m.split(" ").collect();
            let num: u16 = parts[0].parse().expect("broken game");
            let color = parts[1];

            // println!("Color: {}, Num: {}", color, num);
            match color {
                "red" => result.red = num,
                "green" => result.green = num,
                "blue" => result.blue = num,
                _ => {}
            }
        }


        // println!("{:?}", result);

        result
    }
}

fn game_id_to_int(game_string: &str) -> u16 {
    let game_id = game_string.split(" ").collect::<Vec<_>>()[1];

    game_id.parse().expect("every game must have an id")
}

fn process_moves(moves: &str) -> Vec<GameMove> {
    let parts: Vec<_> = moves.split(";").collect();
    parts.into_iter().map(GameMove::from_string).collect()
    // for part in &parts {
    //     // println!("{}", part);
    //     GameMove::from_string(part);
    // }
}

fn is_valid(moves: &Vec<GameMove>) -> bool {
    // 12 red cubes, 13 green cubes, and 14 blue cubes
    let mut result = true;
    for m in moves {
        if m.red > 12 || m.green > 13 || m.blue > 14 {
            result = false;
        }
    }

    result
}

fn min_color(moves: &Vec<GameMove>) -> u64 {
    let reds: Vec<_> = moves.into_iter().map(|x| x.red).collect();
    let greens: Vec<_> = moves.into_iter().map(|x| x.green).collect();
    let blues: Vec<_> = moves.into_iter().map(|x| x.blue).collect();

    let max_red = reds.iter().max().unwrap();
    let max_green = greens.iter().max().unwrap();
    let max_blue = blues.iter().max().unwrap();
    // println!("{max_red}, {max_green}, {max_blue}");

    (max_red * max_green * max_blue) as u64

}

fn main() {
    println!("Hello, from day 2!");
    // let filename = "input/day2-test.txt";
    let filename = "input/day2.txt";

    let mut bad_game_id_sum = 0;
    let mut min_powers = 0;

    for line in read_to_string(filename).unwrap().lines() {
        // println!("{}", line);
        let parts: Vec<_> = line.split(":").collect();
        let game = parts[0];
        let moves = parts[1];
        let game_id = game_id_to_int(&game);
        

        // println!("{}", game_id);
        let gamemoves = process_moves(&moves);
        if is_valid(&gamemoves) {
            bad_game_id_sum += game_id;
        }

        min_powers += min_color(&gamemoves);
    }
    println!("{}", bad_game_id_sum);
    println!("{}", min_powers);
}
