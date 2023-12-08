use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::util::time;

pub fn day5() {
    println!("== Day 5 ==");
    let input = "src/day5/input.txt";
    time(part_a, input, "A");
    time(part_b, input, "B");
}

#[derive(Debug, Clone, Copy)]
struct Mapping {
    destination_start: usize,
    source_start: usize,
    range: usize,
}

impl Mapping {
    pub fn from(row: Vec<usize>) -> Mapping {
        Mapping {
            destination_start: *row.get(0).unwrap(),
            source_start: *row.get(1).unwrap(),
            range: *row.get(2).unwrap(),
        }
    }
}

#[derive(Debug)]
struct SeedMap {
    seeds: Vec<usize>,
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,

}

impl SeedMap {
    pub fn from(input: &Vec<Vec<String>>) -> SeedMap {
        let seeds = input.get(0)
            .map(|s| s.first().unwrap().split_once(": ").unwrap())
            .map(|(_a, b)| b.split(" ").collect::<Vec<&str>>())
            .map(|v| v.iter().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>())
            .unwrap();


        SeedMap {
            seeds: seeds,
            seed_to_soil: input.get(1).map(SeedMap::mappings).unwrap(),
            soil_to_fertilizer: input.get(2).map(SeedMap::mappings).unwrap(),
            fertilizer_to_water: input.get(3).map(SeedMap::mappings).unwrap(),
            water_to_light: input.get(4).map(SeedMap::mappings).unwrap(),
            light_to_temperature: input.get(5).map(SeedMap::mappings).unwrap(),
            temperature_to_humidity: input.get(6).map(SeedMap::mappings).unwrap(),
            humidity_to_location: input.get(7).map(SeedMap::mappings).unwrap(),
        }
    }
    fn mappings(group: &Vec<String>) -> Vec<Mapping> {
        group.iter().skip(1)
            .map(|s| s.split(" ").collect::<Vec<&str>>())
            .map(|v| v.iter().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>())
            .map(|v| Mapping::from(v))
            .collect()
    }

    fn convert(start: usize, map: &Vec<Mapping>) -> usize {
        for mapping in map.iter() {
            // println!("Start: {} mapping: {:?}",start, mapping);
            if mapping.source_start <= start && start <= mapping.source_start + mapping.range {
                let steps = start - mapping.source_start;
                let i = mapping.destination_start + steps;
                // println!("HIT! -- Start: {} mapping: {:?} == Returning: {}",start, mapping, i);
                // println!("MATCH {:?} || {:?} || {:?}",mapping, i, start);
                return i;
            }
        }
        // println!("Not found, returning start-value {}", start);
        start
    }

    fn convert_range(seed: (usize, usize), map: &Vec<Mapping>) -> (usize, usize) {
        for mapping in map.iter() {
            let map_range = (mapping.source_start, mapping.source_start + mapping.range);
            // println!("Seed: {:?}, map_range: {:?}, mapping: {:?}",seed, map_range, mapping);
            if !(max(seed.0, map_range.0) > min(seed.1, map_range.1)) {
                return (0, 0);
            }
        }
        println!("Not found, returning start-value {:?}", seed);
        seed
    }

    fn find_min_location(&self) -> usize {
        self.seeds.iter()
            .map(|seed| SeedMap::convert(*seed, &self.seed_to_soil))
            .map(|n| SeedMap::convert(n, &self.soil_to_fertilizer))
            .map(|n| SeedMap::convert(n, &self.fertilizer_to_water))
            .map(|n| SeedMap::convert(n, &self.water_to_light))
            .map(|n| SeedMap::convert(n, &self.light_to_temperature))
            .map(|n| SeedMap::convert(n, &self.temperature_to_humidity))
            .map(|n| SeedMap::convert(n, &self.humidity_to_location))
            .min()
            .unwrap()
    }
    fn find_min_range_bf(&self) -> usize {
        let chunks: Vec<&[usize]> = self.seeds.chunks(2).collect();

        let mut fin_loc = usize::MAX;
        for chunk in chunks.iter() {
            for i in 0..chunk[1] {
                let seed = chunk[0] + i;
                let loc = Some(seed)
                    .map(|seed| SeedMap::convert(seed, &self.seed_to_soil))
                    .map(|n| SeedMap::convert(n, &self.soil_to_fertilizer))
                    .map(|n| SeedMap::convert(n, &self.fertilizer_to_water))
                    .map(|n| SeedMap::convert(n, &self.water_to_light))
                    .map(|n| SeedMap::convert(n, &self.light_to_temperature))
                    .map(|n| SeedMap::convert(n, &self.temperature_to_humidity))
                    .map(|n| SeedMap::convert(n, &self.humidity_to_location))
                    .unwrap();
                if loc < fin_loc {
                    fin_loc = loc;
                }
            }
        }
        fin_loc
    }
    fn find_min_range_location(&self) -> usize {
        let chunks: Vec<&[usize]> = self.seeds.chunks(2).collect();
        // let new_seeds: Vec<usize> = chunks.iter()
        //     .map(|v| v[0]..v[0]+v[1])
        //     .map(|v| v.collect::<Vec<usize>>())
        //     .flatten()
        //     .collect();
        // println!("{}",new_seeds.len());
        let p = chunks.iter()
            .map(|seed| SeedMap::convert_range((seed[0], seed[0] + seed[1]), &self.seed_to_soil))
            .map(|n| SeedMap::convert_range(n, &self.soil_to_fertilizer))
            .map(|n| SeedMap::convert_range(n, &self.fertilizer_to_water))
            .map(|n| SeedMap::convert_range(n, &self.water_to_light))
            .map(|n| SeedMap::convert_range(n, &self.light_to_temperature))
            .map(|n| SeedMap::convert_range(n, &self.temperature_to_humidity))
            .map(|n| SeedMap::convert_range(n, &self.humidity_to_location))
            .collect::<Vec<(usize, usize)>>();
        // .map(|n| n.0)
        // .min()
        // .unwrap()
        println!("{:?}", p);
        0
    }
}

fn part_a(input: &str) -> usize {
    let open = File::open(input).expect("Could not read file");
    let groups = group_input(open);
    let map = SeedMap::from(&groups);
    map.find_min_location()
}

fn group_input(open: File) -> Vec<Vec<String>> {
    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut group = Vec::new();
    for line in BufReader::new(open).lines() {
        let line = line.unwrap();
        if line.is_empty() {
            groups.push(group);
            group = Vec::new();
        } else {
            group.push(line);
        }
    }
    if !group.is_empty() {
        groups.push(group);
    }
    groups
}

fn part_b(input: &str) -> usize {
    let open = File::open(input).expect("Could not read file");
    let groups = group_input(open);
    let map = SeedMap::from(&groups);
    // map.find_min_range_location()
    map.find_min_range_bf()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn runday() {
        day5();
    }

    #[ignore]
    #[test]
    fn real_a() {
        let input = "src/day5/input.txt";
        assert_eq!(165788812, part_a(input));
    }

    #[ignore]
    #[test]
    fn real_b() {
        let input = "src/day5/input.txt";
        assert_eq!(0, part_b(input));
    }

    #[test]
    fn part_a_test_input() {
        let input = "src/day5/test-input.txt";
        let result = part_a(input);
        assert_eq!(35, result);
    }

    #[test]
    fn part_b_test_input() {
        let input = "src/day5/test-input.txt";
        let result = part_b(input);
        assert_eq!(46, result);
    }

    #[test]
    fn poo() {
        let input = "src/day5/test-input.txt";
        let open = File::open(input).expect("Could not read file");
        let groups = group_input(open);
        let map = SeedMap::from(&groups);
        let _bf = map.find_min_range_bf();
        println!();
        let range = map.find_min_range_location();
        assert_eq!(46, range);
    }

    #[test]
    fn convert_test() {
        let mut simples = Vec::new();
        let i = SeedMap::convert(93, &vec![Mapping::from(vec![52, 50, 48])]);
        simples.push(i);
        let i = SeedMap::convert(i, &vec![Mapping::from(vec![18, 25, 70])]);
        simples.push(i);
        let i = SeedMap::convert(i, &vec![Mapping::from(vec![68, 64, 13])]);
        simples.push(i);
        let i = SeedMap::convert(i, &vec![Mapping::from(vec![60, 56, 37])]);
        simples.push(i);
        let i = SeedMap::convert(i, &vec![Mapping::from(vec![52, 50, 48])]);
        simples.push(i);


        let mut complex = Vec::new();
        let i = SeedMap::convert_range((79, 93), &vec![Mapping::from(vec![52, 50, 48])]);
        complex.push(i);
        let i = SeedMap::convert_range(i, &vec![Mapping::from(vec![18, 25, 70])]);
        complex.push(i);
        let i = SeedMap::convert_range(i, &vec![Mapping::from(vec![68, 64, 13])]);
        complex.push(i);
        let i = SeedMap::convert_range(i, &vec![Mapping::from(vec![60, 56, 37])]);
        complex.push(i);
        let i = SeedMap::convert_range(i, &vec![Mapping::from(vec![52, 50, 48])]);
        complex.push(i);

        println!("{:?}", simples);
        println!("{:?}", complex);
    }

    #[test]
    fn rt() {
        let simple_a = SeedMap::convert(74, &vec![Mapping::from(vec![68, 64, 13])]);
        let simple_b = SeedMap::convert(88, &vec![Mapping::from(vec![68, 64, 13])]);
        let range = SeedMap::convert_range((74, 88), &vec![Mapping::from(vec![68, 64, 13])]);

        println!("Simple: {:?}, Range {:?}", (simple_a, simple_b), range);

        assert_eq!((simple_a, simple_b), range)
    }
}