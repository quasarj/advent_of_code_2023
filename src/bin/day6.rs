struct Boat {
    // speed of the boat in mm/ms/ms button is held
    speed: u64,
}

impl Boat {
    fn hold_button(&self, hold_ms: u64, race_duration_ms: u64) -> u64 {
        if hold_ms >= race_duration_ms {
            0
        } else {
            let current_speed = self.speed * hold_ms;
            current_speed * (race_duration_ms - hold_ms)
        }
    }
}

fn ways_to_win(boat: &Boat, duration_ms: u64, distance_mm: u64) -> u64 {
    let mut ways = 0;

    for i in 0..=duration_ms {
        // println!("{}", i);
        if boat.hold_button(i, duration_ms) > distance_mm {
            // println!("This way can win");
            ways += 1;
        }
    }

    ways
}

fn calc_race(races: &Vec<u64>, boat: &Boat) -> u64 {
    let mut final_score = 1;

    for race in races.chunks(2) {
        let (time, distance) = (race[0], race[1]);
        final_score *= ways_to_win(boat, time, distance);
    }

    final_score
}

fn main() {
    println!("Day 6");

    let myboat = Boat { speed: 1 };
    // Time:        48     87     69     81
    // Distance:   255   1288   1117   1623
    // part races1
    let races_p1 = vec![48, 255, 87, 1288, 69, 1117, 81, 1623];

    // part 2 (probably not brute forcable - I was laughably wrong, took less than 1s lol)
    let races_p2 = vec![48876981, 255128811171623];

    println!("Part 1: {}", calc_race(&races_p1, &myboat));
    println!("Part 2: {}", calc_race(&races_p2, &myboat));
}

#[test]
fn main_myboat() {
    let myboat = Boat { speed: 1 };

    assert_eq!(myboat.hold_button(0, 7), 0);
    assert_eq!(myboat.hold_button(1, 7), 6);
    assert_eq!(myboat.hold_button(2, 7), 10);
    assert_eq!(myboat.hold_button(3, 7), 12);
    assert_eq!(myboat.hold_button(4, 7), 12);
    assert_eq!(myboat.hold_button(5, 7), 10);
    assert_eq!(myboat.hold_button(6, 7), 6);
    assert_eq!(myboat.hold_button(7, 7), 0);
}
