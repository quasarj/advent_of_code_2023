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
    fn get_mapping(&self, num: u64) -> u64 {
        (num - self.src_start) + self.dst_start
    }
}

#[derive(Debug)]
struct Mapping {
    items: Vec<Map>,
}

impl Mapping {
    fn get_mapping(&self, num: u64) -> u64 {
        for map in &self.items {
            if map.contains(num) {
                return map.get_mapping(num);
            }
        }
        return num;
    }
}


fn main() {
    println!("Day 5");

    let mut seed_soil = Mapping {
        items: Vec::new()
    };
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

    println!("{}", seed_soil.get_mapping(98));
    println!("{}", seed_soil.get_mapping(99));
    println!("{}", seed_soil.get_mapping(100));
    // println!("{}", seed_soil.items[0].contains(100));
    // println!("{}", seed_soil.items[0].get_mapping(99));

    // println!("{:?}", seed_soil);
}
