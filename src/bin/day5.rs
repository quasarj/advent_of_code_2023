use log;
use logos::Logos;
// use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
enum Token {
    // Tokens can be literal strings, of any length.
    #[token("seeds:")]
    Seeds,

    #[token("seed-to-soil map:")]
    MapSeedToSoil,

    #[token("soil-to-fertilizer map:")]
    MapSoilToFertilizer,

    #[token("fertilizer-to-water map:")]
    MapFertilizerToWater,

    #[token("water-to-light map:")]
    MapWaterToLight,

    #[token("light-to-temperature map:")]
    MapLightToTemperature,

    #[token("temperature-to-humidity map:")]
    MapTemperatureToHumidity,

    #[token("humidity-to-location map:")]
    MapHumidityToLocation,

    // Or regular expressions.
    #[regex("\\d+")]
    Number,
}

#[derive(Debug)]
struct Map {
    dst_start: u64,
    src_start: u64,
    length: u64,
}
impl Map {
    fn contains(&self, num: u64) -> bool {
        if num >= self.src_start && num < (self.src_start + self.length) {
            true
        } else {
            false
        }
    }
    fn get_mapping(&self, num: u64) -> Option<u64> {
        if num >= self.src_start && num < (self.src_start + self.length) {
            Some((num - self.src_start) + self.dst_start)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Mapping {
    items: Vec<Map>,
}

impl Mapping {
    fn new() -> Self {
        Mapping { items: Vec::new() }
    }

    fn apply(&self, num: u64) -> u64 {
        for map in &self.items {
            match map.get_mapping(num) {
                Some(res) => return res,
                None => continue,
            }
        }
        return num;
    }

    fn from_vec(values: &Vec<u64>) -> Self {
        let mut m = Self::new();
        for chunk in values.chunks(3) {
            println!("chunk: {:?}", chunk);
            m.items.push(Map {
                dst_start: chunk[0],
                src_start: chunk[1],
                length: chunk[2],
            });
        }
        m
    }
}

fn main2() {
    println!("Day 5");

    let mut seed_soil = Mapping { items: Vec::new() };
    // let mut seed_soil = Mapping::new();

    seed_soil.items.push(Map {
        dst_start: 50,
        src_start: 98,
        length: 2,
    });
    seed_soil.items.push(Map {
        dst_start: 52,
        src_start: 50,
        length: 48,
    });

    let mut soil_fert = Mapping { items: Vec::new() };

    soil_fert.items.push(Map {
        dst_start: 0,
        src_start: 15,
        length: 37,
    });
    soil_fert.items.push(Map {
        dst_start: 37,
        src_start: 52,
        length: 2,
    });
    soil_fert.items.push(Map {
        dst_start: 39,
        src_start: 0,
        length: 15,
    });

    let seed_num = 50;
    let soil_num = seed_soil.apply(seed_num);
    let fert_num = soil_fert.apply(soil_num);

    println!("{}", seed_num);
    println!("{}", soil_num);
    println!("{}", fert_num);

    // println!("{:?}", seed_soil);
}

#[derive(Debug)]
enum State {
    Start,
    Seed,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

fn main() {
    env_logger::init();

    let filename = "input/day5.txt";
    // let filename = "input/day5-test.txt";

    log::info!("Using filename={filename}");

    let input_file = read_to_string(filename).unwrap();
    let lines = input_file.lines();

    let mut state = State::Start;
    // TODO this is definitely NOT the best way to do this
    let mut seed_values: Vec<u64> = Vec::new();
    let mut seed_to_soil: Vec<u64> = Vec::new();
    let mut soil_to_fert: Vec<u64> = Vec::new();
    let mut fert_to_water: Vec<u64> = Vec::new();
    let mut water_to_light: Vec<u64> = Vec::new();
    let mut light_to_temp: Vec<u64> = Vec::new();
    let mut temp_to_humid: Vec<u64> = Vec::new();
    let mut humid_to_loc: Vec<u64> = Vec::new();

    for line in lines {
        // println!("{:?}", line);

        let mut lex = Token::lexer(line);

        loop {
            let token = lex.next();
            let value = lex.slice();

            // println!("{:?}", token);
            match token {
                Some(Ok(Token::Seeds)) => state = State::Seed,
                Some(Ok(Token::MapSeedToSoil)) => state = State::SeedToSoil,
                Some(Ok(Token::MapSoilToFertilizer)) => state = State::SoilToFertilizer,
                Some(Ok(Token::MapFertilizerToWater)) => state = State::FertilizerToWater,
                Some(Ok(Token::MapWaterToLight)) => state = State::WaterToLight,
                Some(Ok(Token::MapLightToTemperature)) => state = State::LightToTemperature,
                Some(Ok(Token::MapTemperatureToHumidity)) => state = State::TemperatureToHumidity,
                Some(Ok(Token::MapHumidityToLocation)) => state = State::HumidityToLocation,
                Some(Ok(Token::Number)) => {
                    // println!("{:?} {}", state, value);
                    match state {
                        State::Seed => seed_values.push(value.parse().unwrap()),
                        State::SeedToSoil => seed_to_soil.push(value.parse().unwrap()),
                        State::SoilToFertilizer => soil_to_fert.push(value.parse().unwrap()),
                        State::FertilizerToWater => fert_to_water.push(value.parse().unwrap()),
                        State::WaterToLight => water_to_light.push(value.parse().unwrap()),
                        State::LightToTemperature => light_to_temp.push(value.parse().unwrap()),
                        State::TemperatureToHumidity => temp_to_humid.push(value.parse().unwrap()),
                        State::HumidityToLocation => humid_to_loc.push(value.parse().unwrap()),
                        _ => {}
                    }
                }
                None => break,
                Some(Err(_)) => todo!(),
            }
        }
    }

    println!("{:?}", seed_values);
    println!("{:?}", seed_to_soil);

    // let mut seed_soil_map = Mapping::new();
    // seed_soil_map.load_from_vec(&seed_to_soil);

    let seed_soil_map = Mapping::from_vec(&seed_to_soil);
    let soil_fert_map = Mapping::from_vec(&soil_to_fert);
    let fert_water_map = Mapping::from_vec(&fert_to_water);
    let water_light_map = Mapping::from_vec(&water_to_light);
    let light_temp_map = Mapping::from_vec(&light_to_temp);
    let temp_humid_map = Mapping::from_vec(&temp_to_humid);
    let humid_loc_map = Mapping::from_vec(&humid_to_loc);

    let maps = vec![
        seed_soil_map,
        soil_fert_map,
        fert_water_map,
        water_light_map,
        light_temp_map,
        temp_humid_map,
        humid_loc_map,
    ];

    // let seed_locations: Vec<_> = seed_values.iter().map(|&x| find_seed_loc(x, &maps)).collect();
    // println!("{}", seed_locations.iter().min().unwrap());
    for chunk in seed_values.chunks(2) {
        for i in chunk[0]..(chunk[0] + chunk[1]) {
            find_seed_loc(i, &maps);
        }
        // println!("{:?}", chunk);
    }
}

fn find_seed_loc(seed: u64, maps: &Vec<Mapping>) -> u64 {
    let mut val = seed;
    for map in maps {
        val = map.apply(val);
    }
    val
}
